use meilisearch_types::Document;
use serde_json::json;

use crate::search::insert_geo_distance;

#[test]
fn test_insert_geo_distance() {
    let value: Document = serde_json::from_str(
        r#"{
          "_geo": {
            "lat": 50.629973371633746,
            "lng": 3.0569447399419567
          },
          "city": "Lille",
          "id": "1"
        }"#,
    )
    .unwrap();

    let sorters = &["_geoPoint(50.629973371633746,3.0569447399419567):desc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    let sorters = &["_geoPoint(50.629973371633746, 3.0569447399419567):asc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    let sorters = &["_geoPoint(   50.629973371633746   ,  3.0569447399419567   ):desc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    let sorters = &[
        "prix:asc",
        "villeneuve:desc",
        "_geoPoint(50.629973371633746, 3.0569447399419567):asc",
        "ubu:asc",
    ]
    .map(|s| s.to_string());
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    // only the first geoPoint is used to compute the distance
    let sorters = &[
        "chien:desc",
        "_geoPoint(50.629973371633746, 3.0569447399419567):asc",
        "pangolin:desc",
        "_geoPoint(100.0, -80.0):asc",
        "chat:asc",
    ]
    .map(|s| s.to_string());
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    // there was no _geoPoint so nothing is inserted in the document
    let sorters = &["chien:asc".to_string()];
    let mut document = value;
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), None);
}

#[test]
fn test_insert_geo_distance_with_coords_as_string() {
    let value: Document = serde_json::from_str(
        r#"{
          "_geo": {
            "lat": "50",
            "lng": 3
          }
        }"#,
    )
    .unwrap();

    let sorters = &["_geoPoint(50,3):desc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    let value: Document = serde_json::from_str(
        r#"{
          "_geo": {
            "lat": "50",
            "lng": "3"
          },
          "id": "1"
        }"#,
    )
    .unwrap();

    let sorters = &["_geoPoint(50,3):desc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));

    let value: Document = serde_json::from_str(
        r#"{
          "_geo": {
            "lat": 50,
            "lng": "3"
          },
          "id": "1"
        }"#,
    )
    .unwrap();

    let sorters = &["_geoPoint(50,3):desc".to_string()];
    let mut document = value.clone();
    insert_geo_distance(sorters, &mut document);
    assert_eq!(document.get("_geoDistance"), Some(&json!(0)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{SearchTiming, SearchTimingHierarchy};
    use std::collections::BTreeMap;
    use std::time::Duration;

    #[test]
    fn test_search_timing_hierarchy() {
        // Create a sample SearchTiming with some data
        let mut timing = SearchTiming {
            total_search_time: Duration::from_millis(100),
            query_processing_time: Duration::from_millis(20),
            search_execution_time: Duration::from_millis(50),
            result_formatting_time: Duration::from_millis(30),
            query_parsing_time: Duration::from_millis(10),
            query_tree_build_time: Duration::from_millis(5),
            query_tree_simplify_time: Duration::from_millis(5),
            ranking_graph_build_time: Duration::from_millis(0),
            tokenization_time: Duration::from_millis(15),
            tokenizer_build_time: Duration::from_millis(5),
            keyword_search_time: Duration::from_millis(25),
            vector_search_time: Duration::from_millis(0),
            hybrid_search_time: Duration::from_millis(0),
            embedding_time: Duration::from_millis(0),
            words_ranking_time: Duration::from_millis(10),
            typo_ranking_time: Duration::from_millis(5),
            proximity_ranking_time: Duration::from_millis(5),
            attribute_ranking_time: Duration::from_millis(5),
            exactness_ranking_time: Duration::from_millis(0),
            sort_ranking_time: Duration::from_millis(0),
            geo_sort_time: Duration::from_millis(0),
            geo_filter_time: Duration::from_millis(0),
            geo_sort_compute_time: Duration::from_millis(0),
            geo_bucket_sort_time: Duration::from_millis(0),
            facet_distribution_time: Duration::from_millis(10),
            facet_stats_time: Duration::from_millis(5),
            facet_search_time: Duration::from_millis(0),
            result_sorting_time: Duration::from_millis(15),
            distinct_processing_time: Duration::from_millis(10),
            pagination_time: Duration::from_millis(5),
            cache_lookup_time: Duration::from_millis(5),
            database_read_time: Duration::from_millis(20),
            filter_application_time: Duration::from_millis(10),
            facet_attribute_timing: BTreeMap::new(),
            filter_attribute_timing: BTreeMap::new(),
            additional_timing: BTreeMap::new(),
        };

        // Add some per-attribute timing
        timing.facet_attribute_timing.insert("category".to_string(), Duration::from_millis(8));
        timing.facet_attribute_timing.insert("brand".to_string(), Duration::from_millis(7));

        // Convert to hierarchical format
        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        // Verify the hierarchical structure
        assert!(hierarchy.timing_entries.contains_key("total_search"));
        assert!(hierarchy.timing_entries.contains_key("query_processing"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > parsing"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > tree_build"));
        assert!(hierarchy.timing_entries.contains_key("query_processing > tree_simplify"));
        assert!(hierarchy.timing_entries.contains_key("tokenization"));
        assert!(hierarchy.timing_entries.contains_key("tokenization > tokenizer_build"));
        assert!(hierarchy.timing_entries.contains_key("search_execution"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > keyword_search"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > ranking > words"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > ranking > typo"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > ranking > proximity"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > ranking > attribute"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > cache > lookup"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > database > read"));
        assert!(hierarchy.timing_entries.contains_key("search_execution > database > filter"));
        assert!(hierarchy.timing_entries.contains_key("result_formatting"));
        assert!(hierarchy.timing_entries.contains_key("result_formatting > sorting"));
        assert!(hierarchy.timing_entries.contains_key("result_formatting > distinct"));
        assert!(hierarchy.timing_entries.contains_key("result_formatting > pagination"));
        assert!(hierarchy.timing_entries.contains_key("facets"));
        assert!(hierarchy.timing_entries.contains_key("facets > distribution"));
        assert!(hierarchy.timing_entries.contains_key("facets > stats"));
        assert!(hierarchy.timing_entries.contains_key("facets > attribute > category"));
        assert!(hierarchy.timing_entries.contains_key("facets > attribute > brand"));

        // Verify timing values
        assert_eq!(hierarchy.timing_entries["total_search"], Duration::from_millis(100));
        assert_eq!(hierarchy.timing_entries["query_processing"], Duration::from_millis(20));
        assert_eq!(hierarchy.timing_entries["query_processing > parsing"], Duration::from_millis(10));
        assert_eq!(hierarchy.timing_entries["search_execution"], Duration::from_millis(50));
        assert_eq!(hierarchy.timing_entries["result_formatting"], Duration::from_millis(30));
        assert_eq!(hierarchy.timing_entries["facets"], Duration::from_millis(15)); // 10 + 5 + 0
        assert_eq!(hierarchy.timing_entries["facets > attribute > category"], Duration::from_millis(8));
        assert_eq!(hierarchy.timing_entries["facets > attribute > brand"], Duration::from_millis(7));

        // Verify that zero-duration entries are not included
        assert!(!hierarchy.timing_entries.contains_key("query_processing > ranking_graph_build"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > vector_search"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > hybrid_search"));
    }

    #[test]
    fn test_search_timing_hierarchy_empty() {
        // Create an empty SearchTiming
        let timing = SearchTiming {
            total_search_time: Duration::ZERO,
            query_processing_time: Duration::ZERO,
            search_execution_time: Duration::ZERO,
            result_formatting_time: Duration::ZERO,
            query_parsing_time: Duration::ZERO,
            query_tree_build_time: Duration::ZERO,
            query_tree_simplify_time: Duration::ZERO,
            ranking_graph_build_time: Duration::ZERO,
            tokenization_time: Duration::ZERO,
            tokenizer_build_time: Duration::ZERO,
            keyword_search_time: Duration::ZERO,
            vector_search_time: Duration::ZERO,
            hybrid_search_time: Duration::ZERO,
            embedding_time: Duration::ZERO,
            words_ranking_time: Duration::ZERO,
            typo_ranking_time: Duration::ZERO,
            proximity_ranking_time: Duration::ZERO,
            attribute_ranking_time: Duration::ZERO,
            exactness_ranking_time: Duration::ZERO,
            sort_ranking_time: Duration::ZERO,
            geo_sort_time: Duration::ZERO,
            geo_filter_time: Duration::ZERO,
            geo_sort_compute_time: Duration::ZERO,
            geo_bucket_sort_time: Duration::ZERO,
            facet_distribution_time: Duration::ZERO,
            facet_stats_time: Duration::ZERO,
            facet_search_time: Duration::ZERO,
            result_sorting_time: Duration::ZERO,
            distinct_processing_time: Duration::ZERO,
            pagination_time: Duration::ZERO,
            cache_lookup_time: Duration::ZERO,
            database_read_time: Duration::ZERO,
            filter_application_time: Duration::ZERO,
            facet_attribute_timing: BTreeMap::new(),
            filter_attribute_timing: BTreeMap::new(),
            additional_timing: BTreeMap::new(),
        };

        // Convert to hierarchical format
        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        // Should only contain total_search with zero duration
        assert_eq!(hierarchy.timing_entries.len(), 1);
        assert!(hierarchy.timing_entries.contains_key("total_search"));
        assert_eq!(hierarchy.timing_entries["total_search"], Duration::ZERO);
    }

    #[test]
    fn test_search_timing_hierarchy_display() {
        // Create a sample SearchTiming with some data
        let mut timing = SearchTiming {
            total_search_time: Duration::from_millis(150),
            query_processing_time: Duration::from_millis(30),
            search_execution_time: Duration::from_millis(80),
            result_formatting_time: Duration::from_millis(40),
            query_parsing_time: Duration::from_millis(15),
            query_tree_build_time: Duration::from_millis(10),
            query_tree_simplify_time: Duration::from_millis(5),
            ranking_graph_build_time: Duration::from_millis(0),
            tokenization_time: Duration::from_millis(20),
            tokenizer_build_time: Duration::from_millis(8),
            keyword_search_time: Duration::from_millis(35),
            vector_search_time: Duration::from_millis(0),
            hybrid_search_time: Duration::from_millis(0),
            embedding_time: Duration::from_millis(0),
            words_ranking_time: Duration::from_millis(15),
            typo_ranking_time: Duration::from_millis(8),
            proximity_ranking_time: Duration::from_millis(7),
            attribute_ranking_time: Duration::from_millis(10),
            exactness_ranking_time: Duration::from_millis(0),
            sort_ranking_time: Duration::from_millis(0),
            geo_sort_time: Duration::from_millis(0),
            geo_filter_time: Duration::from_millis(0),
            geo_sort_compute_time: Duration::from_millis(0),
            geo_bucket_sort_time: Duration::from_millis(0),
            facet_distribution_time: Duration::from_millis(12),
            facet_stats_time: Duration::from_millis(6),
            facet_search_time: Duration::from_millis(0),
            result_sorting_time: Duration::from_millis(20),
            distinct_processing_time: Duration::from_millis(12),
            pagination_time: Duration::from_millis(8),
            cache_lookup_time: Duration::from_millis(8),
            database_read_time: Duration::from_millis(25),
            filter_application_time: Duration::from_millis(12),
            facet_attribute_timing: BTreeMap::new(),
            filter_attribute_timing: BTreeMap::new(),
            additional_timing: BTreeMap::new(),
        };

        // Add some per-attribute timing
        timing.facet_attribute_timing.insert("category".to_string(), Duration::from_millis(10));
        timing.facet_attribute_timing.insert("brand".to_string(), Duration::from_millis(8));

        // Convert to hierarchical format
        let hierarchy = SearchTimingHierarchy::from_flat_timing(&timing);

        // Print the hierarchical structure for demonstration
        println!("\n=== Search Timing Hierarchy ===");
        for (key, duration) in &hierarchy.timing_entries {
            let indent = key.matches('>').count() * 2;
            let indent_str = " ".repeat(indent);
            println!("{}{}: {:?}", indent_str, key, duration);
        }
        println!("===============================\n");

        // Verify the structure is correct
        assert_eq!(hierarchy.timing_entries["total_search"], Duration::from_millis(150));
        assert_eq!(hierarchy.timing_entries["query_processing"], Duration::from_millis(30));
        assert_eq!(hierarchy.timing_entries["query_processing > parsing"], Duration::from_millis(15));
        assert_eq!(hierarchy.timing_entries["query_processing > tree_build"], Duration::from_millis(10));
        assert_eq!(hierarchy.timing_entries["query_processing > tree_simplify"], Duration::from_millis(5));
        assert_eq!(hierarchy.timing_entries["tokenization"], Duration::from_millis(20));
        assert_eq!(hierarchy.timing_entries["tokenization > tokenizer_build"], Duration::from_millis(8));
        assert_eq!(hierarchy.timing_entries["search_execution"], Duration::from_millis(80));
        assert_eq!(hierarchy.timing_entries["search_execution > keyword_search"], Duration::from_millis(35));
        assert_eq!(hierarchy.timing_entries["search_execution > ranking > words"], Duration::from_millis(15));
        assert_eq!(hierarchy.timing_entries["search_execution > ranking > typo"], Duration::from_millis(8));
        assert_eq!(hierarchy.timing_entries["search_execution > ranking > proximity"], Duration::from_millis(7));
        assert_eq!(hierarchy.timing_entries["search_execution > ranking > attribute"], Duration::from_millis(10));
        assert_eq!(hierarchy.timing_entries["search_execution > cache > lookup"], Duration::from_millis(8));
        assert_eq!(hierarchy.timing_entries["search_execution > database > read"], Duration::from_millis(25));
        assert_eq!(hierarchy.timing_entries["search_execution > database > filter"], Duration::from_millis(12));
        assert_eq!(hierarchy.timing_entries["result_formatting"], Duration::from_millis(40));
        assert_eq!(hierarchy.timing_entries["result_formatting > sorting"], Duration::from_millis(20));
        assert_eq!(hierarchy.timing_entries["result_formatting > distinct"], Duration::from_millis(12));
        assert_eq!(hierarchy.timing_entries["result_formatting > pagination"], Duration::from_millis(8));
        assert_eq!(hierarchy.timing_entries["facets"], Duration::from_millis(18)); // 12 + 6 + 0
        assert_eq!(hierarchy.timing_entries["facets > distribution"], Duration::from_millis(12));
        assert_eq!(hierarchy.timing_entries["facets > stats"], Duration::from_millis(6));
        assert_eq!(hierarchy.timing_entries["facets > attribute > category"], Duration::from_millis(10));
        assert_eq!(hierarchy.timing_entries["facets > attribute > brand"], Duration::from_millis(8));

        // Verify that zero-duration entries are not included
        assert!(!hierarchy.timing_entries.contains_key("query_processing > ranking_graph_build"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > vector_search"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > hybrid_search"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > ranking > exactness"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > ranking > sort"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > geo > sort"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > geo > filter"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > geo > sort > compute"));
        assert!(!hierarchy.timing_entries.contains_key("search_execution > geo > sort > bucket"));
        assert!(!hierarchy.timing_entries.contains_key("facets > search"));
    }
}
