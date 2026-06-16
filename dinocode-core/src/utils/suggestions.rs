// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/suggestions.rs
//  Desc:       Utility for generating suggestions based on available options.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::cmp;

pub struct SuggestionEngine {
    cache: HashMap<String, Vec<String>>,
}

impl SuggestionEngine {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn find_best_suggestion(&mut self, input: &str, options: &[String]) -> Option<String> {
        let cache_key = format!("{}:{}", input, options.len());
        
        if let Some(cached_suggestions) = self.cache.get(&cache_key) {
            return cached_suggestions.first().cloned();
        }

        let best_match = options
            .iter()
            .map(|option| (option, self.levenshtein_distance(input, option)))
            .min_by_key(|(_, distance)| *distance)
            .filter(|(_, distance)| *distance <= 3)
            .map(|(option, _)| option.clone());

        if let Some(ref match_str) = best_match {
            self.cache.insert(cache_key, vec![match_str.clone()]);
        }

        best_match
    }

    pub fn find_suggestions(&mut self, input: &str, options: &[String], max_results: usize) -> Vec<String> {
        let cache_key = format!("multi:{}:{}", input, options.len());
        
        if let Some(cached_suggestions) = self.cache.get(&cache_key) {
            return cached_suggestions.clone();
        }

        let mut suggestions: Vec<(String, usize)> = options
            .iter()
            .map(|option| (option.clone(), self.levenshtein_distance(input, option)))
            .filter(|(_, distance)| *distance <= 3)
            .collect();

        suggestions.sort_by_key(|(_, distance)| *distance);

        let result: Vec<String> = suggestions
            .into_iter()
            .take(max_results)
            .map(|(option, _)| option)
            .collect();

        self.cache.insert(cache_key, result.clone());

        result
    }


    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = cmp::min(
                    cmp::min(
                        matrix[i - 1][j] + 1,      // deletion
                        matrix[i][j - 1] + 1       // insertion
                    ),
                    matrix[i - 1][j - 1] + cost   // substitution
                );
            }
        }

        matrix[len1][len2]
    }

}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub fn suggest_best_match(input: &str, options: &[String]) -> Option<String> {
    let mut engine = SuggestionEngine::new();
    engine.find_best_suggestion(input, options)
}

