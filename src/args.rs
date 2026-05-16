use crate::types::Position;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// 入力画像ファイルのパス
    pub imagepath: Option<String>,

    /// 切り出し後の画像を出力するフォルダパス
    #[arg(short = 'd', long)]
    pub outdir: Option<String>,

    /// 出力画像ファイル名
    #[arg(short, long)]
    pub outimage: Option<String>,

    /// 画像の切り出し位置
    #[arg(short, long)]
    pub position: Option<Position>,

    /// 切り出す画像の最大幅
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    pub size: Option<u32>,

    /// 入力画像の幅がsize以下だった場合にその大きさまで拡大するか
    #[arg(short, long, num_args = 0..=1, default_missing_value = "true")]
    pub expand: Option<bool>,

    /// 読み込む設定ファイル(デフォルトはプログラムと同じフォルダのimgsquare.conf)
    #[arg(short, long)]
    pub config: Option<String>,
}
