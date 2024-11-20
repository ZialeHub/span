[![Rust CI](https://github.com/ryse-rs/span/actions/workflows/ci-main.yaml/badge.svg)](https://github.com/ryse-rs/span/actions/workflows/ci-main.yaml)
[![codecov](https://codecov.io/gh/ryse-rs/span/graph/badge.svg?token=E7HLJRBTXZ)](https://codecov.io/gh/ryse-rs/span)

# What is span?

Span is a rust time management library. It encapsulates commonly used libs such as `chrono` and adds up nice features.

Span allows to create, update, compare and display custom formats of `Time`, `Date` or `Datetime` with ease.

Each enum represents a different unit of time:
- `TimeUnit::Hour/Minute/Second`
- `DateUnit::Year/Month/Day`
- `DateTimeUnit::Year/Month/Day/Hour/Minute/Second`

# Builder

We provide a `SpanBuilder` to set a custom date format. This leads to less boilerplate for each date calls, and improved consistency through the entire application.
Formats are defined based on a `strftime` inspired date and time formatting syntax.

Here is the default format for each type:
- Time => `"%H:%M:%S"`
- Date => `"%Y-%m-%d"`
- DateTime => `format!("{} {}", BASE_DATE_FORMAT, BASE_TIME_FORMAT)`

# Features

By default _span_ can be used to manage `time`, `date` or `datetime`, but you're free to select features for your app.

- (Default)`["full"]`
- `["time"]`
- `["date"]`
- `["datetime"]`
