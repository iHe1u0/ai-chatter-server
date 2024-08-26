use mysql::prelude::Queryable;
use mysql::{params, Pool};
use rocket::serde::{Deserialize, Serialize};

/// `User` 结构体表示一个用户对象，包含用户的基本信息。
///
/// 该结构体派生了多个 trait:
/// - `Debug`: 允许使用 `{:?}` 格式输出用户对象，用于调试。
/// - `Serialize` 和 `Deserialize`: 使 `User` 结构体可以被序列化和反序列化，以便在不同格式（如 JSON）之间转换。
/// - `FromForm`: 允许从表单数据创建 `User` 对象，用于在 Web 应用程序中处理用户输入。
#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct User {
    /// 用户的唯一标识符，通常用于区分不同用户。
    pub id: Option<usize>,

    /// 用户的账号名称，用于登录系统。
    pub account: String,

    /// 用户的密码，建议存储为加密形式以确保安全。
    pub password: String,

    /// 用户是否存在的标志，用于标识用户状态。
    pub exist: Option<bool>,
}

/// 用户结构体的实现部分
impl User {
    /// 创建一个新的 `User` 实例。
    ///
    /// # 参数
    ///
    /// - `id`: 用户的唯一标识符。
    /// - `account`: 用户的账号名称。
    /// - `password`: 用户的密码。
    /// - `exist`: 一个布尔值，表示用户是否存在。
    ///
    /// # 返回
    ///
    /// 返回一个包含给定信息的 `User` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// let user = User::new(1, String::from("test_account"), String::from("test_password"), true);
    /// ```
    pub fn new(id: usize, account: String, password: String, exist: bool) -> User {
        // 使用给定的 id、account、password 和 exist 值创建一个 User 实例
        User {
            id: Some(id),
            account,
            password,
            exist: Some(exist),
        }
    }

    pub fn validate(&self) -> bool {
        let database_url = "mysql://chat_db_user:chatUdb2024@192.168.0.109:3306/chat_db";

        // 建立数据库连接
        let pool = Pool::new(database_url).expect("Failed to create a connection pool.");
        let mut conn = pool
            .get_conn()
            .expect("Failed to get a connection from the pool.");
        // 执行查询
        let res = conn.exec_first(
            "SELECT id, account, password, exist FROM user WHERE account = :account AND password = :password",
            params! {
        "account" => &self.account,
        "password" => &self.password,
    },
        ).map(|row| {
            row.map(|(id, account, password, exist)| User {
                id,
                account,
                password,
                exist,
            })
        });

        // 使用 `if let` 来处理 `Option<Result<User>>`，避免多次 `unwrap`
        if let Ok(Some(user)) = res {
            if user.exist.expect("What the fuck") {
                return true;
            }
        }
        false
    }
}
