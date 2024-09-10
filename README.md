# Subgraph: `Antelope Blocks`

> Full history blocks
>
> WAX, EOS, Ultra, Telos...
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1)

- [x] **Blocks**

## Chains

- **API Key**: https://thegraph.com/studio/apikeys/
- **Base URL**: https://gateway.thegraph.com/api
- **Query URL format**: `{base_url}`/api/`{api-key}`/subgraphs/id/`{subgraph_id}`

| Chain | Subgraph ID |
| ----- | ----------- |
| WAX   | [`??`](https://thegraph.com/explorer/subgraphs/???view=Query&chain=arbitrum-one) |
| EOS   | [`??`](https://thegraph.com/explorer/subgraphs/???view=Query&chain=arbitrum-one) |

## GraphQL

```graphql
blocks{
  number
  time
}
```

## Substreams Modules

```mermaid
graph TD;
  graph_out[map: graph_out];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> graph_out;
```