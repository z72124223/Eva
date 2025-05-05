//! scene_scoring.rs
//! 劇情品質自評工具，給出 0~1 分與理由。
use std::collections::HashSet;

/// 計算 N-gram 重複率
fn ngram_repetition(text: &str, n: usize) -> f32 {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.len() < n { return 0.0; }
    let mut ngrams = HashSet::new();
    let mut repeat = 0;
    for i in 0..=tokens.len()-n {
        let ng = &tokens[i..i+n];
        let key = ng.join(" ");
        if !ngrams.insert(key) { repeat += 1; }
    }
    if tokens.len() == 0 { 0.0 } else { repeat as f32 / tokens.len() as f32 }
}

/// 劇情張力停滯（簡化：句子平均長度/變化）
fn tension_flatness(text: &str) -> f32 {
    let sents: Vec<&str> = text.split(['。', '.', '！', '!', '?', '？']).collect();
    if sents.len() < 2 { return 0.0; }
    let lens: Vec<usize> = sents.iter().map(|s| s.chars().count()).collect();
    let mean = lens.iter().sum::<usize>() as f32 / lens.len() as f32;
    let var = lens.iter().map(|l| (*l as f32 - mean).abs()).sum::<f32>() / lens.len() as f32;
    1.0 - (var / mean.max(1.0)).min(1.0) // 越趨近 1 表示越平
}

/// 角色對話迴圈（簡化：連續同角色說話次數）
fn dialogue_loop(text: &str) -> f32 {
    let mut prev = "";
    let mut loop_count = 0;
    for line in text.lines() {
        if let Some((who, _)) = line.split_once(':') {
            if who == prev { loop_count += 1; }
            prev = who;
        }
    }
    (loop_count as f32 / text.lines().count().max(1) as f32).min(1.0)
}

/// 綜合評分
pub fn score_scene(scene: &str) -> (f32, String) {
    let rep = ngram_repetition(scene, 4);
    let flat = tension_flatness(scene);
    let loopf = dialogue_loop(scene);
    let mut score = 1.0 - (rep + flat + loopf) / 3.0;
    if score < 0.0 { score = 0.0; }
    let reason = format!("Ngram重複:{:.2}, 張力平:{:.2}, 對話loop:{:.2}", rep, flat, loopf);
    (score, reason)
}
