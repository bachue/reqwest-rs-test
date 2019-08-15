fn main() {
    let result = reqwest::get("http://api.qiniu.com/v2/query?ak=0p6YmsTFa-MLL4cZhe2Yj8n7nXZ3N_wk0ZmuZMnu&bucket=z5-bucket");
    dbg!(result.unwrap_err());
}
