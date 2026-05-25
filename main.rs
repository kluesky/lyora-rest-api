mod account;
mod bank;

use bank::Bank;

fn main() {
    let mut bank = Bank::new();

    let acc1 = bank.create_account("Budi".to_string(), 1000000.0);
    let acc2 = bank.create_account("Ani".to_string(), 500000.0);

    println!("🏦 Rekening dibuat:");
    println!("{:?}", acc1);
    println!("{:?}", acc2);

    // Deposit
    if let Some(acc) = bank.get_account_mut(1) {
        acc.deposit(250000.0).unwrap();
        println!("\n💰 Setelah deposit: {:?}", acc);
    }

    // Withdraw
    if let Some(acc) = bank.get_account_mut(2) {
        match acc.withdraw(100000.0) {
            Ok(_) => println!("✅ Penarikan berhasil: {:?}", acc),
            Err(e) => println!("❌ {}", e),
        }
    }

    println!("\n📊 Total Aset Bank: Rp{:.2}", bank.total_assets());
}
