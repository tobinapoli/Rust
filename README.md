<div style="background-color:#f4f4f4; padding:15px; border-radius:5px; font-family:monospace; white-space:pre-wrap;">
<h1 style="font-size: 24px; font-weight: bold;">📌 Club SemRust - Payment Registration and Activity Management System on Blockchain</h1>

## 📝 Description
This project implements a blockchain-based system for registering monthly membership payments and managing sports activities for the members of Club SemRust. It does so through two SmartContracts, one main contract for the club's functionality and another that generates reports using information from the first one.

## 🚀 Features
1. **New Member Registration:**
   - When registering a new member, the corresponding category must be selected.
   - A pending payment is created with a due date within the next 10 days.

2. ** Payment Registration:**
   - Members can make their monthly payment by providing their identification number (DNI) and the amount paid.
   - The system verifies that the amount paid corresponds to the member's category and records the payment on the blockchain.

3. ** Payment Inquiry:**
   - It is possible to check the list of completed payments, displaying the member's information, category, and amount paid.

4. ** Bonus for Consecutive Payments:**
   - If a member accumulates a certain number of consecutive on-time payments, a discount is granted for the next month's membership fee.

## 📊 Reports (through another contract)
1. ** Pending Payments Verification:**
   - Displays a list of delinquent members (with outstanding payments past the due date).

2. ** Revenue Report:**
   - Generates a monthly report with the total revenue collected for each membership category.

3. ** Non-Delinquent Members Report for a Specific Activity:**
   - Generates a report of non-delinquent members who are allowed to attend a specific sports activity.

## 🔐 Contracts
- A list of authorized addresses is maintained to perform operations.
- The contract owner can authorize or deauthorize addresses.
- The authorization policy can be enabled or disabled.
- The contract owner can delegate authority to another address.

## ✅ Testing and Documentation
- 🛠️ Testing performed on all functionalities (85% coverage).
- 📚 Documentation provided for all structs and methods.
</div>

