# DevLoop

**The RPG that levels you up by coding. Not by grinding in-game.**

---

## What is DevLoop?

DevLoop is a terminal-based idle RPG for programmers. You have stats, loot, pets, dungeons, quests—but here’s the twist: **you can’t level up by playing the game**. Real-world actions like coding, committing, building projects, or contributing to GitHub are what make your character grow.

DevLoop uses **DSV (Dash Separated Values)** for fast, type-safe communication and struct storage between its services, plugins, and the main game process.

---

## Why DevLoop?

- Can’t find motivation to code?  
- Want coding to feel like a game, not a chore?  
- Love stats, progression, and RPG mechanics?  

DevLoop rewards your real-world programming skills. Every commit, build, pull request, or contribution counts. Fail? You might lose XP.

---

## Features

- Idle RPG with shops, NPCs, pets, skills, and dungeons  
- Custom todo lists and quests  
- Standalone plugins for GitHub, IDEs, and compilers  
- Background service for webhooks (smee.io)  
- Real-time or deferred XP tracking (FIFO + pending file)  
- Fully modular and customizable  
- **DSV-based communication**: FIFO, packs, atoms, metadata for safe, structured data transfer  

---

## How DSV Works in DevLoop

1. **Events** (coding activity, commits, builds) are serialized as **DSV packs**.  
2. Each pack contains **atoms**, e.g., XP gained, loot earned, quest progress.  
3. Packs include optional metadata: `[META: Packs: N, ID: X, SIZE: Y]`, `[SECRET: token]`, `[SHA256: hash]`.  
4. Multi-pack events are reconstructed by the main game safely.  
5. Flexible typing ensures `int`, `float`, `bool`, `string`, arrays, and enums are safely transferred between Rust/C services.

**Example DSV Pack for XP Event:**

```
[META: Packs: 1, ID: 101, SIZE: 64]
--- START ---
xp_gained: 25
level_up: false
loot: ["wooden_sword", "potion"]
--- END ---
```

**Example DSV Pack with Optional Metadata:**

```
[META: Packs: 2, ID: 102, SIZE: 128]
[SECRET: abcd1234]
[SHA256: e3b0c44298fc1c149afbf4c8996fb924]
--- START ---
xp_gained: 50
coins: 10
--- END ---
--- START ---
loot: ["iron_sword"]
pets: ["cat", "dragon"]
--- END ---
```

This allows DevLoop to process live events from multiple sources safely, maintaining **type safety and cross-language compatibility**.

---

## Who is it for?

- **Beginners** → stay motivated, avoid burnout  
- **Casual devs** → fun and rewarding gameplay  
- **Hardcore devs** → optimize, level up, and compete with your own stats  

---

## Getting Started

1. Install the service and plugins (click-to-install, no setup)  
2. Run the game (`game.exe`)  
3. Code, build, commit, contribute, and watch your character grow  

---

## Planned Features

- More IDE/plugin integrations (VSCode, JetBrains, Neovim, Make, etc.)  
- Achievements tied to open-source contributions  
- Leaderboards and social hooks for devs  
- Extended DSV metadata support for advanced pack tracking  

---

> **Note:** DevLoop is **meant for programmers**. If you don’t code, your character won’t level up.

---

## Screenshots / Demo

*(Add screenshots or GIFs here to immediately show XP/loot in action... I still dont have them... BARE WITH ME)*

