pub mod graph {
    use crate::Record;
    use std::collections::VecDeque;
    #[allow(dead_code,non_snake_case)]
    pub struct Graph {
        pub Nodes: Vec<bool>,
        pub GraphMatrix: Vec<Vec<i32>>,
    }

    #[allow(dead_code,non_snake_case)]
    pub fn new(records: &Vec<Record>) -> Graph {
        let mut nodes = Vec::new();
        for record in records {
            if record.Attrition == "Yes" {
                nodes.push(true);
            } else {
                nodes.push(false);
            }
        }
        let mut graphMatrix = Vec::new();
        let lengthOfRecords = records.len();
        for i in 0..lengthOfRecords {
            let mut graphRow = Vec::new();
            for j in 0..lengthOfRecords {
                if i == j{
                    graphRow.push(0);
                    continue;
                }    
                let simularity = calculateSimularity(&records[i], &records[j]);
                if simularity > 40 {
                    graphRow.push(0);
                } else {
                    graphRow.push(simularity);
                }
            }
            graphMatrix.push(graphRow)
        }
        Graph {
            Nodes: nodes,
            GraphMatrix: graphMatrix,
        }
    }

    #[allow(dead_code,non_snake_case)]
    pub fn calculateSimularity(record1: &Record, record2: &Record) -> i32{
        //∆ age + point if dept same + ∆distance +  ∆edu + ∆ environment satisfaction + if gender same point + if jobrole same point + ∆ years in curr role + ∆ stock option level + ∆ years at company
        let deltaAge = (record1.Age.unwrap() - record2.Age.unwrap()).abs();
        let mut departmentPoint = 0;
        if record1.Department != record2.Department {
            departmentPoint = 1;
        }
        let deltaDistance = (record1.DistanceFromHome.unwrap() - record2.DistanceFromHome.unwrap()).abs();
        let deltaEducation = (record1.Education.unwrap() - record2.Education.unwrap()).abs();
        let deltaEnvironment: i32 = (record1.EnvironmentSatisfaction.unwrap() - record2.EnvironmentSatisfaction.unwrap()).abs();
        let mut genderPoint = 0;
        if record1.Gender != record2.Gender {
            genderPoint = 1;
        }
        let deltaJobLevel = (record1.JobLevel.unwrap() - record2.JobLevel.unwrap()).abs();
        let deltaYearsInRole = (record1.YearsInCurrentRole.unwrap() - record2.YearsInCurrentRole.unwrap()).abs();
        let deltaStock = (record1.StockOptionLevel.unwrap() - record2.StockOptionLevel.unwrap()).abs();
        let deltaYearsAtCompany = (record1.YearsAtCompany.unwrap() - record2.YearsAtCompany.unwrap()).abs();
        
        deltaAge + departmentPoint + deltaDistance + deltaEducation + deltaEnvironment + genderPoint + deltaJobLevel + deltaYearsInRole + deltaStock + deltaYearsAtCompany
    }

    pub fn graph_analysis(graph: &Graph, start: usize) -> Option<i32>{
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; graph.Nodes.len()];

        visited[start] = true;
        queue.push_back((start, 0));

        let mut closest_distance: Option<i32> = None;
        
        while let Some((node, distance)) = queue.pop_front() {
            if graph.Nodes[node] {
                closest_distance = Some(distance);
                break;
            }
            for (adjacent_nodes, &edge) in graph.GraphMatrix[node].iter().enumerate() {
                if edge != 0 && !visited[adjacent_nodes] {
                    queue.push_back((adjacent_nodes, distance+1));
                    visited[adjacent_nodes] = true;
                }
            }
        }
        closest_distance
    }
}