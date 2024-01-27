# API Docs

## Endpoints

GET /rooms
desc: get rooms
POST /rooms
desc: create rooms

## Auth

## Flows

1. user create room with POST /rooms
2. user start sending message to room they created earlier with POST /chat, data: {room_name: "room:abc", chat: "test"}

# Data

## Retrieval

```
> redis-cli

## Users by room
SMEMBERS chatrs:room:abc
```
