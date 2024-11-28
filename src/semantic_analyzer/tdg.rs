use std::collections::{HashMap, HashSet, VecDeque};

use super::*;

pub struct TDG<'a> {
    tasks: Vec<(&'a str, TaskType)>,
    methods: Vec<(&'a Symbol<'a>, HTN<'a>)>,
    edges_from_tasks: HashMap<usize, HashSet<usize>>,
    edges_to_tasks: HashMap<usize, HashSet<usize>>,
}

impl<'a> TDG<'a> {
    pub fn new(domain: &'a DomainAST<'a>) -> TDG<'a> {
        // collect task names
        let mut tasks: Vec<(&str, TaskType)> = vec![];
        tasks.extend(
            domain
                .compound_tasks
                .iter()
                .map(|x| (x.name, TaskType::Compound)),
        );
        tasks.extend(domain.actions.iter().map(|x| (x.name, TaskType::Primitive)));

        // edges
        let mut to_methods = HashMap::new();
        let mut to_tasks = HashMap::new();

        // compute index of tasks and methods for efficiency
        let mut task_indices = HashMap::new();
        for (index, (task, _)) in tasks.iter().enumerate() {
            task_indices.insert(*task, index);
            to_methods.insert(index, HashSet::new());
        }

        let mut methods = vec![];
        // collect "task to method" edges
        for (method_index, method) in domain.methods.iter().enumerate() {
            methods.push((&method.name, method.tn.clone()));
            match task_indices.get(method.task.name) {
                Some(task_index) => match to_methods.get_mut(task_index) {
                    Some(set) => {
                        set.insert(method_index);
                    }
                    None => panic!("{} not found", task_index),
                },
                None => panic!("{} is not defined", method.task.name),
            }
        }

        // collect "method to task" edges
        for (method_index, method) in methods.iter().enumerate() {
            let tasks: HashSet<usize> = method
                .1
                .subtasks
                .iter()
                .map(|x| match task_indices.get(x.task.name) {
                    Some(id) => *id,
                    None => panic!("{} not found", x.task.name),
                })
                .collect();
            to_tasks.insert(method_index, tasks);
        }
        TDG {
            tasks: tasks,
            methods: methods,
            edges_from_tasks: to_methods,
            edges_to_tasks: to_tasks,
        }
    }

    pub fn reachable(&self, task_name: &str) -> ReachableSet {
        let mut reach_t = HashSet::new();
        let task_index = match self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, (name, _))| *name == task_name)
            .next()
            .unwrap() {
                // if primitive, the only reachable task is itself
                (_, (name, TaskType::Primitive)) => {
                    return ReachableSet {
                        primitives: HashSet::from([*name]),
                        compounds: HashSet::new(),
                        nullable: false
                    };
                }
                // if compound, add the index for further processing
                (i, (_, TaskType::Compound)) => {
                    i
                }
            };
        reach_t.insert(task_index);
        let mut visited= HashSet::new();
        let mut queue = VecDeque::from([task_index]);
        while !queue.is_empty() {
            let task = queue.pop_front().unwrap();
            if !visited.contains(&task) {
                visited.insert(task);
                match self.edges_from_tasks.get(&task) {
                    Some(methods) => {
                        for m in methods {
                            let new_tasks = self.edges_to_tasks.get(m).unwrap();
                            for new_task in new_tasks.iter() {
                                reach_t.insert(*new_task);
                                queue.push_back(*new_task);
                            }
                        }
                    }
                    None => { }
                }
            }
            
        }
        
        let nullables = self.compute_nullables();
        let mut primitives = HashSet::new();
        let mut compounds = HashSet::new();
        for (index, (reachable_name, reachable_type)) in self.tasks.iter().enumerate() {
            if reach_t.contains(&index) {
                match reachable_type {
                    TaskType::Primitive => {
                        primitives.insert(*reachable_name);
                    }
                    TaskType::Compound => {
                        compounds.insert(*reachable_name);
                    }
                }
            }
        }
        ReachableSet {
            primitives,
            compounds,
            nullable: nullables.contains(task_name),
        }
    }

    pub fn get_recursion_type(&self, nullable_symbols: &HashSet<&'a str>) -> RecursionType {
        let nullables: HashSet<usize> = nullable_symbols
            .iter()
            .map(|x| self.get_task_index(&x))
            .collect();
        let mut recursion_type = RecursionType::NonRecursive;
        // DFS over TDG
        let mut stack = vec![];
        // initiating the stack
        // TODO: restrict to those reachable from inital task network
        for (t, methods) in self.edges_from_tasks.iter() {
            for method in methods {
                stack.push(vec![(*t, *method)]);
            }
        }
        // induction
        while let Some(path) = stack.pop() {
            let (_, current_method) = path.iter().last().unwrap();
            for new_task in self.edges_to_tasks.get(current_method).unwrap() {
                let mut cycle: Option<Vec<(usize, usize)>> = None;
                for (index, (t, _)) in path.iter().enumerate() {
                    if t == new_task {
                        let mut cycle_path: Vec<(usize, usize)> =
                            path.iter().skip(index).cloned().collect();
                        cycle_path.push((*new_task, *current_method));
                        cycle = Some(cycle_path);
                        break;
                    }
                }
                match cycle {
                    Some(cyclic_path) => {
                        // compute cycle prefix
                        let mut is_epsilon_prefix = true;
                        let mut suffix: Vec<usize> = vec![];
                        for (index, (t_id, _)) in cyclic_path.iter().skip(1).enumerate() {
                            let (_, m_id) = cyclic_path[index];
                            let prefix = self.get_prefix(*t_id, m_id);
                            // check epsilon prefix
                            if prefix.len() > 0 {
                                if prefix[0] != *new_task {
                                    for x in prefix.iter() {
                                        if !nullables.contains(x) {
                                            is_epsilon_prefix = false;
                                            break;
                                        }
                                    }
                                }
                            }
                            suffix.extend(self.get_suffix(*t_id, m_id));
                        }
                        // convert cyclic path to names
                        let cyclic_path = cyclic_path
                            .iter()
                            .map(|(task_id, method_id)| {
                                (
                                    self.tasks[*task_id].0.to_string(),
                                    self.methods[*method_id].0.name.to_string(),
                                )
                            })
                            .collect();
                        if is_epsilon_prefix == true {
                            if suffix.len() == 0 {
                                match recursion_type {
                                    RecursionType::GrowAndShrinkRecursion(_) => {}
                                    _ => {
                                        recursion_type = RecursionType::EmptyRecursion(cyclic_path);
                                    }
                                }
                            } else {
                                let nullable_suffix =
                                    suffix.iter().all(|sym| nullables.contains(sym));
                                match recursion_type {
                                    RecursionType::GrowAndShrinkRecursion(_) => {}
                                    RecursionType::EmptyRecursion(_) => {
                                        if nullable_suffix {
                                            recursion_type =
                                                RecursionType::GrowAndShrinkRecursion(cyclic_path);
                                        }
                                    }
                                    _ => {
                                        if nullable_suffix {
                                            recursion_type =
                                                RecursionType::GrowAndShrinkRecursion(cyclic_path);
                                        } else {
                                            recursion_type =
                                                RecursionType::GrowingEmptyPrefixRecursion(
                                                    cyclic_path,
                                                );
                                        }
                                    }
                                }
                            }
                        } else {
                            match recursion_type {
                                RecursionType::NonRecursive => {
                                    recursion_type = RecursionType::Recursive(cyclic_path);
                                }
                                _ => {}
                            }
                        }
                    }
                    None => {
                        if let Some(methods) = self.edges_from_tasks.get(new_task) {
                            for method in methods {
                                let mut new_path = path.clone();
                                new_path.push((*new_task, *method));
                                stack.push(new_path);
                            }
                        }
                    }
                }
            }
        }
        return recursion_type;
    }

    fn get_prefix(&self, task_index: usize, method_index: usize) -> Vec<usize> {
        let (_, method) = &self.methods[method_index];
        let (task, _) = &self.tasks[task_index];
        match &method.orderings {
            TaskOrdering::Total => {
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if subtask.task.name == *task {
                        return method
                            .subtasks
                            .iter()
                            .take(index)
                            .map(|x| self.get_task_index(&x.task.name))
                            .collect();
                    }
                }
                panic!("{} does not exist in {:?}", task, method.subtasks)
            }
            TaskOrdering::Partial(orderings) => {
                // TODO: generalize to unordered tasks
                assert_eq!(orderings.len(), method.subtasks.len() - 1);
                // construct task id mappings
                let mut id_to_task_mapping: HashMap<&str, &str> = HashMap::new();
                let mut task_occurances: HashSet<&str> = HashSet::new();
                for subtask in method.subtasks.iter() {
                    match &subtask.id {
                        Some(id) => {
                            id_to_task_mapping.insert(&id.name, &subtask.task.name);
                            if subtask.task.name == *task {
                                task_occurances.insert(&id.name);
                            }
                        }
                        None => {}
                    }
                }
                // construct the ordering graph
                let mut adjacency: HashMap<&str, HashSet<&str>> = HashMap::new();
                for (e1, e2) in orderings {
                    if adjacency.contains_key(e1) {
                        let neighbors: &mut HashSet<&str> = adjacency.get_mut(e1).unwrap();
                        neighbors.insert(&e2);
                    } else {
                        adjacency.insert(&e1, HashSet::from([*e2]));
                    }
                }
                // find tasks that are explicitly ordered after "task"
                let mut prefix: Vec<&str> = Vec::new();
                let mut stack = Vec::from_iter(task_occurances.iter());
                while let Some(t) = stack.pop() {
                    match adjacency.get(t) {
                        Some(outgoings) => {
                            for outgoing in outgoings {
                                for occurance in task_occurances.iter() {
                                    if outgoing.contains(occurance) {
                                        prefix.push(id_to_task_mapping.get(t).unwrap());
                                        stack.push(&t);
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                return prefix
                    .iter()
                    .map(|id| {
                        let task_name = id_to_task_mapping.get(id).unwrap();
                        self.get_task_index(&task_name)
                    })
                    .collect();
            }
        }
    }

    fn get_suffix(&self, task_index: usize, method_index: usize) -> Vec<usize> {
        let (_, method) = &self.methods[method_index];
        let (task, _) = &self.tasks[task_index];
        match &method.orderings {
            TaskOrdering::Total => {
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if subtask.task.name == *task {
                        return method
                            .subtasks
                            .iter()
                            .skip(index + 1)
                            .map(|x| self.get_task_index(&x.task.name))
                            .collect();
                    }
                }
                panic!("{} does not exist in {:?}", task, method)
            }
            TaskOrdering::Partial(orderings) => {
                // TODO: generalize to unordered tasks
                assert_eq!(orderings.len(), method.subtasks.len() - 1);
                // construct task id mappings
                let mut id_to_task_mapping: HashMap<&str, &str> = HashMap::new();
                let mut task_occurances: HashSet<&str> = HashSet::new();
                for subtask in method.subtasks.iter() {
                    match &subtask.id {
                        Some(id) => {
                            id_to_task_mapping.insert(&id.name, &subtask.task.name);
                            if subtask.task.name == *task {
                                task_occurances.insert(&id.name);
                            }
                        }
                        None => {}
                    }
                }
                // construct ordering graph
                let mut adjacency: HashMap<&str, HashSet<&str>> = HashMap::new();
                for (e1, e2) in orderings {
                    if adjacency.contains_key(e1) {
                        let neighbors: &mut HashSet<&str> = adjacency.get_mut(e1).unwrap();
                        neighbors.insert(e2);
                    } else {
                        adjacency.insert(e1, HashSet::from([*e2]));
                    }
                }
                // find tasks that are explicitly ordered after "task"
                let mut suffix: Vec<&str> = Vec::new();
                let mut stack = Vec::from_iter(task_occurances.iter());
                while let Some(t) = stack.pop() {
                    match adjacency.get(t) {
                        Some(outgoing) => {
                            stack.extend(outgoing.iter());
                            suffix.extend(outgoing.iter());
                        }
                        None => {}
                    }
                }
                suffix
                    .iter()
                    .map(|id| {
                        let task_name = id_to_task_mapping.get(id).unwrap();
                        self.get_task_index(&task_name)
                    })
                    .collect()
            }
        }
    }

    fn get_task_index(&self, task_name: &str) -> usize {
        self.tasks
            .iter()
            .enumerate()
            .find(|(_, (name, t_type))| *name == task_name)
            .unwrap()
            .0
    }

    pub fn compute_nullables(&self) -> HashSet<&'a str> {
        // nullable base case
        let mut nullables: HashSet<usize> = self
            .edges_from_tasks
            .iter()
            .filter_map(|(task, methods)| {
                for method in methods.iter() {
                    let tasks = self.edges_to_tasks.get(method).unwrap();
                    if tasks.len() == 0 {
                        return Some(*task);
                    }
                }
                None
            })
            .collect();

        // unit reachability base case
        let mut unit_reachability: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (t, t_type) in self.tasks.iter() {
            match *t_type {
                TaskType::Primitive => {}
                TaskType::Compound => {
                    let task_index = self.get_task_index(t);
                    let mut value = HashSet::from([task_index]);
                    if let Some(methods) = self.edges_from_tasks.get(&task_index) {
                        for method in methods {
                            let tasks = self.edges_to_tasks.get(method).unwrap();
                            if tasks.len() == 1 {
                                value.insert(*tasks.iter().next().unwrap());
                            }
                        }
                    }

                    unit_reachability.insert(task_index, value);
                }
            }
        }
        let mut changed_nullables = true;
        let mut changed_unit_reachability = true;
        let mut new_nullables = HashSet::new();
        let mut new_unit_reachable: HashMap<usize, HashSet<usize>> = HashMap::new();
        while changed_nullables || changed_unit_reachability {
            // nullables induction step
            for (t, methods) in self.edges_from_tasks.iter() {
                for method in methods {
                    if let Some(tasks) = self.edges_to_tasks.get(method) {
                        if tasks.iter().all(|x| match unit_reachability.get(x) {
                            Some(set) => {
                                let intersection: HashSet<&usize> =
                                    set.intersection(&nullables).collect();
                                intersection.len() != 0
                            }
                            None => false,
                        }) {
                            new_nullables.insert(*t);
                        }
                    }
                }
            }

            // unit reachability induction step
            for (c, previous_reachables) in unit_reachability.iter() {
                let mut change = previous_reachables.clone();
                for previous_reachable in previous_reachables {
                    match unit_reachability.get(previous_reachable) {
                        Some(tasks) => {
                            change = change.union(tasks).cloned().collect();
                        }
                        None => {}
                    }
                }
                for method in self.edges_from_tasks.get(c).unwrap() {
                    if let Some(tasks) = self.edges_to_tasks.get(method) {
                        let mut not_nullable = None;
                        for task in tasks {
                            if !nullables.contains(task) {
                                if not_nullable.is_none() {
                                    not_nullable = Some(*task)
                                } else {
                                    break;
                                }
                            }
                        }
                        if let Some(val) = not_nullable {
                            change.insert(val);
                        }
                    }
                }
                if change == *previous_reachables {
                    changed_unit_reachability = false;
                } else {
                    new_unit_reachable.insert(*c, change);
                }
            }

            // commit to changes
            //// nullables
            if new_nullables.len() == nullables.len() {
                changed_nullables = false;
            } else {
                for n in new_nullables.iter() {
                    nullables.insert(*n);
                }
            }
            //// unit reachability
            for (task, new_reachable) in new_unit_reachable.iter() {
                let prev = unit_reachability.get_mut(&task).unwrap();
                prev.extend(new_reachable);
            }
        }
        let mut result = HashSet::new();
        for task_index in nullables {
            result.insert(self.tasks[task_index].0);
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum TaskType {
    Primitive,
    Compound,
}

pub struct ReachableSet<'a> {
    pub primitives: HashSet<&'a str>,
    pub compounds: HashSet<&'a str>,
    pub nullable: bool,
}
