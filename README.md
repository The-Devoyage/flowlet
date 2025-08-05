# 🌊 Flowlet

A modern, minimal CLI tool designed to **persist your developer flow** — save, run, organize, and sync shell commands, variables, and workflows with ease. Flowlet helps you reduce friction, automate repeat tasks, and move seamlessly between local and cloud environments.

**NOTE:** Flowlet is an example app built on top of [Deeb DB](https://deebkit.com) that requires a self hosted and experimental Deeb Server running to function correctly.

---

## ✨ Features

- 🔖 Save and organize reusable shell commands  
- 🚀 Run saved commands with variable injection  
- ☁️  Sync with Flowlet Cloud 
- 🧠 Extract and save variables from command output  
- 📁 Save, update, remove, and list commands & vars  
- 📤 Push/pull commands from the cloud
- 📁 Projects for grouping related commands and variables

### 🧪 Maybe Soon

- 🗒️ Notes and annotations for commands
- 👥 Collaboration and sharing (teams, permissions)
- 🌎 Multiple environments (e.g. dev, staging, prod) [coming soon]
- 🧭 Enhanced TUI mode for browsing, running, and editing

---

## 📦 Installation

### 🦀 With Cargo

Coming soon to crates.io... but for now, while in development ----

```bash
cargo install --path .
```

## 🚦 Usage

### 🔖 Save a command

```bash
flowlet command save myCommand "curl -X GET https://api.example.com/data"
```

It's advised not to save secrets, instead save them as variables (not synced to the cloud).

### 📜 List Saved Commands

Keep your commands at your fingertips — and fetch them from the cloud anytime.

```bash
flowlet command ls
flowlet command ls --remote  # Fetch from remote server
```

### 👀 Show a command

```bash
flowlet command show myCommand
```

### 🧪 Run a command

```bash
flowlet command run myCommand
```

With options:

```bash
--save-var mySavedOutput --json-path "auth.token"
```

Example:

```bash
flowlet command run myCommand --save-var myFirstVar #Saves output from command as var.
flowlet command run myCommand --save-var mySecondVar --json-path auth.token #Saves path from json
```

This saves the variable, allowing it to be reused in other commands with ${token}-style placeholders.

<!-- **Hint** -->

<!-- You can run a command with a shorthand syntax: -->

<!-- ```bash -->
<!-- flowlet myCommand -->
<!-- ``` -->

### 🌐 Sync

🔄 Pull a remote command by name

```bash
flowlet command pull myCommand
```

### 📤 Push a command by name

```bash
flowlet command push myCommand
```

### 🔐 Authentication

🆕 Register

```bash
flowlet auth register
```

### 🔓 Login

```bash
flowlet auth login 
```

### 🚪 Logout

```bash
flowlet auth logout
```

### 💡 Variables

📋 List variables

```bash
flowlet vars ls
```

### ➕ Add variable

```bash
flowlet vars set myKey someValue
```

### ❌ Remove variable

```bash
flowlet vars rm myKey
```

### 🪄 Use variable

Then use ${myKey} anywhere in your saved command, like:

```bash
curl -H "Authorization: Bearer ${myKey}" http://url.com
```

## 🛠 Developer Setup

```bash
git clone https://github.com/the-devoyage/flowlet.git
cd flowlet
cargo install --path .
```

Ensure you have a `deeb-server` running with the rules from `./server_rules.rhai`. Instructions coming soon!

## 📄 License

MIT © The Devoyage

## 💬 Feedback

Feel free to open an issue or submit a PR. Flowlet is early but growing fast — would love your ideas and contributions!
