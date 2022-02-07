A app using Tauri + React stack to load flight data

## Get Started
1. Setting Up Your Environment
Using [Tauri documentation](https://tauri.studio/en/docs/getting-started/intro#setting-up-your-environment) for your platform of choice.

2. Clone repo and install package dependencies
```
npm i
```

3. Run frontend app at `/root`
```
npm run start
```

You will see a browser open, but it won't load anything.

4. Run backend tauri app at `/src-tauri`
```
cargo run
```

You will see a new window which loads the desktop app.

## Features
1. Load and display data from a flight file (more than thousand rows).
