use serde::{Deserialize, Serialize};
use serde_json::Value;
use suanleme_sdk::{
    dto::{Config, Request},
    SuanlemeClient,
}; // 假设 SuanlemeClient 是从 suanleme_sdk 中导入的
mod client_util;
use client_util::create_suanleme_client;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Default)]
struct UserScoreRequest {
    phone: String,
}
#[derive(Debug, Default, suanleme_common::suanleme_macro::Data, Serialize, Deserialize)]
struct Postadminadduserscore {
    pub remark: String,
    pub r#type: String,
    pub user_ids: Vec<i64>,
    pub score: i64,
}
// #[tokio::main] // 用 tokio 托管异步主函数（用于调试）
//     match slmv(&client, remark, r#type, user_ids, score).await {
//         Ok(s)  => println!("成功：{}", s),
//         Err(e) => eprintln!("失败：{}", e),
//     }
// }
// #[tokio::main]
#[tauri::command]

async fn slmv(
    remark: String,
    r#type: String,
    user_ids: Vec<i64>,
    rmb: i64,
) -> Result<String, String> {
    let client = create_suanleme_client()?;
    let score = rmb * 100000;
    // 构造请求体
    let request_body = Postadminadduserscore {
        remark,
        r#type,
        user_ids,
        score,
    };
    let req = Request::default()
        .url("/api/admin/add_user_score".to_owned())
        .verify(true)
        .encrypt(false)
        .method(suanleme_sdk::dto::Method::POST)
        .body(Some(request_body));

    // 发起异步请求
    let resp = client
        .execute::<Postadminadduserscore, Value>(req)
        .await
        .map_err(|e| format!("请求失败：{}", e))?;
    // let a = serde_json::to_string(&resp).unwrap();
    // let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    // let ret = std::fs::read_to_string("./resp.txt");
    // if ret.is_err() {
    //     std::fs::write("resp.txt", a + &"\n").expect("无法写入文件");
    // } else {
    //     let mut ret = ret.unwrap();
    //     ret += &now;
    //     ret += &a;
    //     ret += &"\n";
    //     std::fs::write("resp.txt", ret).expect("无法写入文件");
    // }
    Ok(format!("响应：{:#?}", resp))
}

#[derive(Debug, Default, suanleme_common::suanleme_macro::Data, Serialize, Deserialize)]
struct Getmachinesitem {
    pub machine_id: String,
}
#[tauri::command]
async fn getmachinesitem(machine_id: String) -> Result<String, String> {
    let client = create_suanleme_client()?;
    let request_body = Getmachinesitem { machine_id };
    let req = Request::default()
        .url("/api/machines/item".to_owned())
        .verify(true)
        .encrypt(false)
        .method(suanleme_sdk::dto::Method::GET)
        .body(Some(request_body));
    let resp = client
        .execute::<Getmachinesitem, Value>(req)
        .await
        .map_err(|e| format!("ERROR: {}", e))?;
    Ok(format!("响应：{:#?}", resp))
}

#[derive(Debug, Default, suanleme_common::suanleme_macro::Data, Serialize, Deserialize)]
struct Posttasks {
    pub name: String,                           // 任���名称
    pub desc: String,                           //任务描述
    pub peer_income: Option<f64>,               //单位收益
    pub expect_time: Option<String>,            // 预期单位时长
    pub r#type: String,                         //任务类型
    pub runtime: i64,                           // 运行环境
    pub time_required: Option<i64>,             //任务时长
    pub points: i32,                            //任务点数
    pub package: Uuid,                          //任务文件 uuid
    pub cpu_required: Option<i32>,              //硬件要求：核
    pub memory_required: Option<i32>,           //硬件要求：G
    pub disk_required: Option<i32>,             //空间要求：G
    pub gpu_required: Vec<i32>,                 //GPU 要求
    pub is_run_bore: Option<bool>,              //是否运行 bore
    pub domain_prefix: Option<String>,          //域名前缀
    pub is_unrestricted_gpu: bool,              //是否不限制 GPU
    pub is_welfare: bool,                       //是否为福利任务
    pub cuda_version: Option<String>,           //CUDA 版本要求
    pub cuda_version_required: Vec<String>,     //CUDA 版本要求
    pub is_unrestricted_cuda_version: bool,     //是否不限制 CUDA 版本
    pub affinity: Vec<String>,                  //任务节点要求
    pub aversion: Vec<String>,                  //任务节点排除
    pub docker_compose_content: Option<String>, //dockerYml 文件内容
}

#[derive(Debug, Serialize)]
pub struct CommonResponse<T> {
    pub code: String,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[tauri::command]
async fn posttasks(
    name: String,
    desc: String,
    peer_income: Option<f64>,
    expect_time: Option<String>,
    r#type: String,
    runtime: i64,
    time_required: Option<i64>,
    points: i32,
    package: Uuid,
    cpu_required: Option<i32>,
    memory_required: Option<i32>,
    disk_required: Option<i32>,
    gpu_required: Vec<i32>,
    is_run_bore: Option<bool>,
    domain_prefix: Option<String>,
    is_unrestricted_gpu: bool,
    is_welfare: bool,
    cuda_version: Option<String>,
    cuda_version_required: Vec<String>,
    is_unrestricted_cuda_version: bool,
    affinity: Vec<String>,
    aversion: Vec<String>,
    docker_compose_content: Option<String>,
) -> Result<String, String> {
    let client = create_suanleme_client()?;
    let request_body = Posttasks {
        name,
        desc,
        peer_income,
        expect_time,
        r#type,
        runtime,
        time_required,
        points,
        package,
        cpu_required,
        memory_required,
        disk_required,
        gpu_required,
        is_run_bore,
        domain_prefix,
        is_unrestricted_gpu,
        is_welfare,
        cuda_version,
        cuda_version_required,
        is_unrestricted_cuda_version,
        affinity,
        aversion,
        docker_compose_content,
    };
    let req = Request::default()
        .url("/api/tasks".to_owned())
        .verify(true)
        .encrypt(false)
        .method(suanleme_sdk::dto::Method::POST)
        .body(Some(request_body));
    let resp = client
        .execute::<Posttasks, Value>(req)
        .await
        .map_err(|e| format!("ERROR: {}", e))?;
    Ok(format!("响应：{:#?}", resp))
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // 可选插件
        .invoke_handler(tauri::generate_handler![slmv, getmachinesitem,posttasks]) // 注册 Tauri 命令
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用失败");
}
