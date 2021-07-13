use rand::seq::SliceRandom;
use ranking_handler::{fetch, ranking_type::RankingType, seichi_api, util};
use std::{collections::HashMap, process};

const RANKING_URL: &str = "https://ranking-gigantic.seichi.click/api/ranking";

fn queries(ranking_type: RankingType) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("type".to_string(), ranking_type.as_str());
    map.insert("offset".to_string(), "0".to_string());
    map.insert("lim".to_string(), ranking_type.get_targets().to_string());
    map.insert("duration".to_string(), "monthly".to_string());
    map
}

async fn fetch_data(ranking_type: RankingType) -> Vec<seichi_api::Lottery> {
    let result = fetch::fetch(RANKING_URL, Some(queries(ranking_type))).await;
    if let Err(e) = result {
        eprintln!(r"APIとの通信中にエラーが発生しました。\n{}", e);
        process::exit(1);
    }
    let result = seichi_api::deserialize(result.unwrap()).await;
    if let Err(e) = result {
        eprintln!(r"型変換の実行中にエラーが発生しました。\n{}", e);
        process::exit(1);
    }
    let result = result.unwrap().ranks;
    result
        .iter()
        .map(|rank| seichi_api::Lottery::convert(rank))
        .collect()
}

#[tokio::main]
async fn main() {
    println!("月別ランキング報酬の抽選を開始します。");
    println!();

    println!("今月の月別景品付与の対象になったプレイヤーの方々を一覧表示します。");
    util::pause();

    println!();

    println!("整地量：{}名", RankingType::Break.get_targets());
    let break_targets = fetch_data(RankingType::Break).await;
    println!("{:#?}", break_targets);

    println!();

    println!("建築量：{}名", RankingType::Build.get_targets());
    let build_targets = fetch_data(RankingType::Build).await;
    println!("{:#?}", build_targets);

    println!();

    println!("景品が実際に付与される方はこちらです。");
    util::pause();

    println!();

    println!("整地量：{}名", RankingType::Break.get_winners());
    println!();
    let rng = &mut rand::thread_rng();
    let break_winner: Vec<_> = break_targets
        .choose_multiple(rng, RankingType::Break.get_winners().into())
        .cloned()
        .collect();
    println!("{:#?}", break_winner);

    println!();

    println!("建築量：{}名", RankingType::Build.get_winners());
    println!();
    let build_winner: Vec<_> = build_targets
        .choose_multiple(rng, RankingType::Build.get_winners().into())
        .cloned()
        .collect();
    println!("{:#?}", build_winner);

    println!();

    println!("抽選を終了しました。");
}
