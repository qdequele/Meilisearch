#[cfg(test)]
mod tests {
    use crate::search::{SearchTiming, SearchTimingHierarchy};
    use std::time::Duration;

    #[test]
    fn test_search_timing_hierarchy() {
        let mut timing = SearchTiming::default();
        timing.total_search_time = Duration::from_millis(100);
        timing.query_processing_time = Duration::from_millis(30);
        timing.query_parsing_time = Duration::from_millis(10);
        timing.tokenization_time = Duration::from_millis(20);
        timing.search_execution_time = Duration::from_millis(40);
        timing.result_formatting_time = Duration::from_millis(10);

        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        // Check that hierarchical entries exist
        assert!(hierarchy.timing_entries.contains_key("processing_time_ms"));
        assert!(hierarchy.timing_entries.contains_key("query_processing"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > parsing"));
        assert!(hierarchy.timing_entries.contains_key("tokenization"));
        assert!(hierarchy.timing_entries.contains_key("search_execution"));
        assert!(hierarchy.timing_entries.contains_key("result_formatting"));

        // Check values (they're now JSON values, so we need to extract them)
        let processing_time_ms = hierarchy.timing_entries.get("processing_time_ms").unwrap();
        assert_eq!(processing_time_ms.as_str().unwrap(), "100.00ms");

        let query_processing = hierarchy.timing_entries.get("query_processing").unwrap();
        assert_eq!(query_processing.as_str().unwrap(), "30.00ms");

        let query_parsing = hierarchy.timing_entries.get("query_processing > parsing").unwrap();
        assert_eq!(query_parsing.as_str().unwrap(), "10.00ms");
    }

    #[test]
    fn test_search_timing_hierarchy_empty() {
        let timing = SearchTiming::default();
        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        // Only processing_time_ms should be present with zero duration
        assert_eq!(hierarchy.timing_entries.len(), 1);
        assert!(hierarchy.timing_entries.contains_key("processing_time_ms"));
        
        let processing_time_ms = hierarchy.timing_entries.get("processing_time_ms").unwrap();
        assert_eq!(processing_time_ms.as_str().unwrap(), "0.00ns");
    }

    #[test]
    fn test_search_timing_hierarchy_display() {
        let mut timing = SearchTiming::default();
        timing.total_search_time = Duration::from_millis(150);
        timing.query_processing_time = Duration::from_millis(50);
        timing.query_parsing_time = Duration::from_millis(20);
        timing.query_tree_build_time = Duration::from_millis(30);
        timing.tokenization_time = Duration::from_millis(25);
        timing.search_execution_time = Duration::from_millis(60);
        timing.keyword_search_time = Duration::from_millis(40);
        timing.vector_search_time = Duration::from_millis(20);
        timing.result_formatting_time = Duration::from_millis(15);

        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        println!("Search Timing Hierarchy:");
        for (key, value) in &hierarchy.timing_entries {
            println!("  {}: {}", key, value);
        }

        // Verify the hierarchical structure
        assert!(hierarchy.timing_entries.contains_key("processing_time_ms"));
        assert!(hierarchy.timing_entries.contains_key("query_processing"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > parsing"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > tree_build"));
        assert!(hierarchy.timing_entries.contains_key("search_execution"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > keyword_search"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > vector_search"));
    }
}
