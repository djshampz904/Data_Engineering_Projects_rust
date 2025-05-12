# Concurrent Log Parser and Analyzer

## High performance log analyzer that processess large log files concurrently, extracting insights and generate reports

### Technlogies

- Concurrecy: Tokio/ Rayon prallelism and async runtime
- Data processing: nom for parsing, serde for serialization
- Storage: SQLite local storage
- API: axum for exposring results via REST API
- Visualization: Plotters and generating Charts
- CI/CD: Github actiosn

### Functionalitites
- Concurrent log file processing using worker pools
- Pattern recognition for errors and warnings
- Time-series analysis of log events
- Statistical aggregation (counts, distributions, etc.)
- Real-time monitoring capabilities with WebSocket updates
- Customizable alerting rules
- Dashboard for visualization
