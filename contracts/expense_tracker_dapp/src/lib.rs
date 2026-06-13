#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct ExpenseTrackerContract;

#[contractimpl]
impl ExpenseTrackerContract {
    
    // Hàm ghi nhận một khoản chi tiêu mới
    // Bao gồm: category (danh mục - ví dụ: "An uong", "Di lai") và amount (số tiền)
    pub fn add_expense(_env: Env, category: String, amount: u32) {
        // Tạm thời để trống. Sau này sẽ lưu (category, amount) vào blockchain
    }

    // Hàm lấy ra danh mục chi tiêu gần nhất (để test thử)
    pub fn get_latest_category(_env: Env) -> String {
        // Trả về dữ liệu giả lập
        String::from_str(&_env, "An uong")
    }
}