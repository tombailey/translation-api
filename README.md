# translation-api

## Introduction

This is a simple translation API written in Rust.

## Getting started

```dockerfile
FROM tombailey256/translation-api:0.0.0

ENV PORT = 8080
```

See Docker Hub for the latest release tags:
[https://hub.docker.com/r/tombailey256/translation-api](https://hub.docker.com/r/tombailey256/translation-api)

## Rest API

### Translate
```shell
curl -X POST -H "Content-Type: application/json" http://localhost:8080/translate -d '[{ "source": "en", "target": "fr", "input": "Hello" }]'
# 200 OK
# [{ "source": "en", "output": "Bonjour" }]
```

## Providers

### Claude

Uses a [Claude model](https://docs.anthropic.com/claude/docs/models-overview) for translation.

```shell
export CLAUDE_API_KEY="..."
export CLAUDE_API_VERSION="2023-06-01"
export CLAUDE_MODEL="claude-3-sonnet-20240229"
export CLAUDE_MAX_PARALLEL_REQUESTS="3"
```

### DeepL

Uses [DeepL](https://www.deepl.com/pro-api?cta=header-pro-api) for translation.

```shell
export DEEPL_API="https://api-free.deepl.com/v2"
export DEEPL_AUTHENTICATION_KEY="..."
export DEEPL_MAX_PARALLEL_REQUESTS="3"
```

### OpenAI

Uses an [OpenAI model](https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo) for translation.

```shell
export OPENAI_API_KEY="..."
export OPENAI_MODEL="gpt-4-turbo-preview"
export OPENAI_MAX_PARALLEL_REQUESTS="3"
```

## Health check

A built-in health check endpoint (`/health`) confirms that the translation-api is working correctly. Where possible, it will verify connectivity with the specified provider.
