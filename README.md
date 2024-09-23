# 💍 Marriage Platform (*WIP*)

This project is designed to help Shi'a students at our university connect with potential spouses in a safe and respectful environment.

## 🚀 Features

- **Profile Creation**: Users can create and manage their profiles.
- **User Authentication**: Secure sign-up and login mechanisms.
- **Connection Requests**: Users can request contact information with mutual consent.
- **Responsive Design**: Accessible on both desktop and mobile devices.

## ⚙️ Technologies Used

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Axum-3A3A3A?style=for-the-badge&logo=rust&logoColor=white" alt="Axum">
  <img src="https://img.shields.io/badge/SQLx-477999?style=for-the-badge&logo=postgresql&logoColor=white" alt="SQLx">
  <img src="https://img.shields.io/badge/Yew-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Yew">
  <img src="https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL">
</p>


- **Backend**:
  - [Rust](https://www.rust-lang.org/): A safe, concurrent, and practical language.
  - [Axum](https://github.com/tokio-rs/axum): A web framework that focuses on ergonomics and modularity.
  - [SQLx](https://github.com/launchbadge/sqlx): An async SQL toolkit for Rust.
  
- **Frontend**:
  - [Yew](https://yew.rs/): A modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.
  
- **Database**:
  - [PostgreSQL](https://www.postgresql.org/): A powerful, open-source relational database system.

## 📦 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- [PostgreSQL](https://www.postgresql.org/download/) installed and running.

### Clone the Repository

```zsh
git clone https://github.com/smarizvi110/marriage_platform.git
cd marriage_platform
```

### Backend Setup

1. Create a `.env` file in the `backend` directory using the provided [`.env.example`](marriage_platform_backend/.env.example) as a template:

    ```zsh
    cp backend/.env.example backend/.env
    ```

2. Update the `.env` file with your PostgreSQL database credentials.
3. Run the migrations:

   ```zsh
    cd backend
    cargo sqlx migrate run
    ```

4. Start the backend server:

    ```zsh
     cargo run
     ```

### Frontend Setup

1. Build and run the frontend

   ```zsh
   cd frontend
   trunk serve
   ```

2. Open your browser and navigate to [`http://localhost:8080`](http://localhost:8080).

## 🤝 Contributing

We welcome contributions to enhance this project! Here’s how you can contribute:

1. Fork the Repository: Click the fork button to create your copy.
2. Create a Branch:

   ```zsh
    git checkout -b feature/your-feature-name
    ```

3. Make your changes: Implement your feature or fix a bug.
4. Commit your changes:

   ```zsh
    git commit -m "Your commit message"
    ```

5. Push to your fork:

   ```zsh
    git push origin feature/your-feature-name
    ```

6. Create a Pull Request: Open a pull request to the main branch of the original repository with your changes.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to reach out if you have any questions or need assistance.
