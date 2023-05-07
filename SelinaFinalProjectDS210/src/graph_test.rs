#[cfg(test)]

mod tests {
    use crate::Record;
    use crate::graph::graph::{calculateSimularity, new, graph_analysis};
    
    
    fn create_sample_records() -> Vec<Record> {
        vec![
            Record {
                Age: Some(25),
                Attrition: "No".to_string(),
                BusinessTravel: "Travel_Rarely".to_string(),
                DailyRate: Some(200),
                Department: "Research & Development".to_string(),
                DistanceFromHome: Some(1),
                Education: Some(1),
                EducationField: "Life Sciences".to_string(),
                EmployeeCount: Some(1),
                EmployeeNumber: Some(1),
                EnvironmentSatisfaction: Some(2),
                Gender: "Female".to_string(),
                HourlyRate: Some(94),
                JobInvolvement: Some(3),
                JobLevel: Some(2),
                JobRole: "Laboratory Technician".to_string(),
                JobSatisfaction: Some(4),
                MaritalStatus: "Single".to_string(),
                MonthlyIncome: Some(5993),
                MonthlyRate: Some(19479),
                NumCompaniesWorked: Some(8),
                Over18: "Y".to_string(),
                PercentSalaryHike: Some(11),
                PerformanceRating: Some(3),
                RelationshipSatisfaction: Some(1),
                StandardHours: Some(80),
                StockOptionLevel: Some(0),
                TotalWorkingYears: Some(8),
                TrainingTimesLastYear: Some(0),
                WorkLifeBalance: Some(1),
                YearsAtCompany: Some(6),
                YearsInCurrentRole: Some(4),
                YearsSinceLastPromotion: Some(0),
                YearsWithCurrManager: Some(2),
            },

            Record {
                Age: Some(23),
                Attrition: "Yes".to_string(),
                BusinessTravel: "Travel_Frequently".to_string(),
                DailyRate: Some(279),
                Department: "Life Sciences".to_string(),
                DistanceFromHome: Some(8),
                Education: Some(2),
                EducationField: "Life Sciences".to_string(),
                EmployeeCount: Some(1),
                EmployeeNumber: Some(1),
                EnvironmentSatisfaction: Some(2),
                Gender: "Male".to_string(),
                HourlyRate: Some(56),
                JobInvolvement: Some(3),
                JobLevel: Some(2),
                JobRole: "Sales Executive".to_string(),
                JobSatisfaction: Some(4),
                MaritalStatus: "Single".to_string(),
                MonthlyIncome: Some(5993),
                MonthlyRate: Some(19479),
                NumCompaniesWorked: Some(8),
                Over18: "Y".to_string(),
                PercentSalaryHike: Some(11),
                PerformanceRating: Some(3),
                RelationshipSatisfaction: Some(1),
                StandardHours: Some(75),
                StockOptionLevel: Some(2),
                TotalWorkingYears: Some(7),
                TrainingTimesLastYear: Some(0),
                WorkLifeBalance: Some(1),
                YearsAtCompany: Some(6),
                YearsInCurrentRole: Some(3),
                YearsSinceLastPromotion: Some(0),
                YearsWithCurrManager: Some(2),
            },
        ]
    }


#[test]
    fn test_calculate_similarity() {
        let records = create_sample_records();
        let similarity = calculateSimularity(&records[0], &records[1]);
        assert_eq!(similarity, 15);
    }

#[test]
    fn test_graph_analysis() {
        let records = create_sample_records();
        let graph = new(&records);
        let start = 0;
        let closest_distance = graph_analysis(&graph, start);
        assert_eq!(closest_distance, Some(1));
    }

}
    

