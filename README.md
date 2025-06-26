# 🌊 Flowlet

A modern and minimal command-line tool to save, run, organize, and sync your favorite shell commands (and more) — locally and in the cloud.

---

## ✨ Features

- 🔖 Save and organize reusable shell commands  
- 🚀 Run saved commands with variable injection  
- 🔐 Detect secrets before saving accidentally  
- ☁️  Sync with a remote server
- 🧠 Extract and save variables from command output  
- 📁 Save, update, remove, and list commands & vars  
- 🪄 Auto JSON correction for malformd data
- 📤 Push/pull specific or all commands  

### Maybe Soon

- 📦 Pretty and modern terminal UI with `rich`  
- ✅ Register/login with auth token handling  
- 👥 Future support for notes, messaging, and collaboration  

---

## 📦 Installation

### 🐍 With pip (editable dev install)

```bash
git clone https://github.com/yourusername/flowlet.git
cd flowlet
python3 -m venv venv
source venv/bin/activate
pip install -e .
```

## 🧪 Requirements

- Python 3.8+
- `deeb-server` (if self hosted)

Install dependencies:

```bash
pip install -r requirements.txt
```

## 🚦 Usage

### 🔖 Save a command

```bash
flowlet command save myCommand "curl -X GET https://api.example.com/data"
```

If the command contains something that looks like a secret (API keys, tokens, passwords), you’ll get a warning with confirmation.

### 📜 List saved commands

```bash
flowlet command ls
flowlet command ls --remote  # Fetch from server
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
--arg "<extra args here>"
--save-var "token=auth.token"
```

Example:

```bash
flowlet command run myCommand --save-var "token=auth.token"
```

This saves the variable to ~/.flowlet_vars.json, allowing it to be reused in other commands with ${token}-style placeholders.

**Hint**

You can run a command with a shorthand syntax:

```bash
flowlet myCommand
```

### 🌐 Sync

🔄 Pull all remote commands

```bash
flowlet command pull
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
flowlet vars
```

### ➕ Add variable

```bash
flowlet vars add myKey someValue
```

### ❌ Remove variable

```bash
flowlet vars rm myKey
```

### 🪄 Use variable

Then use ${myKey} anywhere in your saved command, like:

```bash
curl -H "Authorization: Bearer ${myKey}"
```

### 🧠 Secrets Detection

Flowlet uses detect-secrets to scan for common secrets before saving commands.

- Warns before saving
- Shows what was detected
- Lets you approve or reject

## 🛠 Developer Setup

```bash
git clone https://github.com/yourusername/flowlet.git
cd flowlet
python3 -m venv venv
source venv/bin/activate
pip install -e .
```

Ensure you have a `deeb-server` running with the rules from `./server_rules.rhai`. Instructions coming soon!

## 📄 License

MIT © The Devoyage

## 💬 Feedback

Feel free to open an issue or submit a PR. Flowlet is early but growing fast — would love your ideas and contributions!
