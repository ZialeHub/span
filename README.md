[![Rust CI](https://github.com/ryse-rs/span/actions/workflows/ci-main.yaml/badge.svg)](https://github.com/ryse-rs/span/actions/workflows/ci-main.yaml)
[![codecov](https://codecov.io/gh/ryse-rs/span/graph/badge.svg?token=E7HLJRBTXZ)](https://codecov.io/gh/ryse-rs/span)

# What is span?

Span is a rust time management library. It encapsulates commonly used libs such as `chrono` and adds up nice features.

Span will allow you create, update, compare and display Time, Date or Datetime with ease.

Each types contains his own Unit type to precisely manage your time span :
- `TimeUnit::Hour/Minute/Second`
- `DateUnit::Year/Month/Day`
- `DateTimeUnit::Year/Month/Day/Hour/Minute/Second`

# Builder

We provide a `SpanBuilder` to set default formats for your app, you won't need to set the format for each variables in your code to display them.
Formats are define based on a `strftime` inspired date and time formatting syntax.

By default formats are :
- Time => `"%H:%M:%S"`
- Date => `"%Y-%m-%d"`
- DateTime => `format!("{} {}", BASE_DATE_FORMAT, BASE_TIME_FORMAT)`

# Features

By default the lib can be used to manage `time`, `date` or `datetime`, but you're free to select features for your app.

- (Default)`["full"]`
- `["time"]`
- `["date"]`
- `["datetime"]`
