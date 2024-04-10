# Rust weather CLI
This is the CLI tool to check for the weather using Rust <br>
It makes API calls to [WeatherAPI.com](https://www.weatherapi.com), and formats the result into a table

# How to run
- Acquire an API token from [WeatherAPI.com](https://www.weatherapi.com/my/)
- Set the environment variable: <br>
    [`.env.sample`](.env.sample)
    ```
    WEATHER_API_TOKEN = "..."
    ```
- Rename `.env.sample` into `.env`
- Build the project
  ```rust
  cargo build --release
  ```
- The `weather` CLI tool will be in `/target/release/weather`. Since Rust static links by default, you can bring this binary anywhere and it will run fine

# How does it look
```
$ ./weather 
Handy dandy tool to check the weather forecast

Usage: weather <COMMAND>

Commands:
  current   Check the current weather
  forecast  Check the forecast weather. Going from 1 - 14 days in the future from now
  future    Check the forecast weather. Going from 14 - 300 days in the future from now
  history   Check the forecast history
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Usage examples
## Normal use
```bash
$ ./weather current -q Hanoi
╭──────────┬──────────────────────────────────────────────────────────────────────────────────────╮
│ current  │ ╭────────────────────┬─────────────────────────────────────────────────────────────╮ │
│          │ │ cloud              │  75                                                         │ │
│          │ ├────────────────────┼─────────────────────────────────────────────────────────────┤ │
│          │ │ condition          │ ╭──────┬──────────────────────────────────────────────────╮ │ │
│          │ │                    │ │ code │  1030                                            │ │ │
│          │ │                    │ ├──────┼──────────────────────────────────────────────────┤ │ │
│          │ │                    │ │ icon │  //cdn.weatherapi.com/weather/64x64/day/143.png  │ │ │
│          │ │                    │ │ text │  Mist                                            │ │ │
│          │ │                    │ ╰──────┴──────────────────────────────────────────────────╯ │ │
│          │ │ feelslike_c        │  25.2                                                       │ │
│          │ │ feelslike_f        │  77.3                                                       │ │
│          │ │ gust_kph           │  7.2                                                        │ │
│          │ │ gust_mph           │  4.5                                                        │ │
│          │ │ humidity           │  94                                                         │ │
│          │ │ is_day             │  1                                                          │ │
│          │ │ last_updated       │  2024-04-10 09:00                                           │ │
│          │ │ last_updated_epoch │  1712714400                                                 │ │
│          │ │ precip_in          │  0.0                                                        │ │
│          │ │ precip_mm          │  0.0                                                        │ │
│          │ │ pressure_in        │  30.0                                                       │ │
│          │ │ pressure_mb        │  1016.0                                                     │ │
│          │ │ temp_c             │  23.0                                                       │ │
│          │ │ temp_f             │  73.4                                                       │ │
│          │ │ uv                 │  5.0                                                        │ │
│          │ │ vis_km             │  5.0                                                        │ │
│          │ │ vis_miles          │  3.0                                                        │ │
│          │ │ wind_degree        │  120                                                        │ │
│          │ │ wind_dir           │  ESE                                                        │ │
│          │ │ wind_kph           │  6.8                                                        │ │
│          │ │ wind_mph           │  4.3                                                        │ │
│          │ ╰────────────────────┴─────────────────────────────────────────────────────────────╯ │
├──────────┼──────────────────────────────────────────────────────────────────────────────────────┤
│ location │ ╭─────────────────┬───────────────────╮                                              │
│          │ │ country         │  Vietnam          │                                              │
│          │ ├─────────────────┼───────────────────┤                                              │
│          │ │ lat             │  21.03            │                                              │
│          │ │ localtime       │  2024-04-10 9:03  │                                              │
│          │ │ localtime_epoch │  1712714591       │                                              │
│          │ │ lon             │  105.85           │                                              │
│          │ │ name            │  Hanoi            │                                              │
│          │ │ region          │                   │                                              │
│          │ │ tz_id           │  Asia/Bangkok     │                                              │
│          │ ╰─────────────────┴───────────────────╯                                              │
╰──────────┴──────────────────────────────────────────────────────────────────────────────────────╯
```
# Save into a file
```bash
$ ./weather current -q Hanoi > current-weather.txt
```