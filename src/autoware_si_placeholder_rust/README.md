# autoware_si_placeholder_rust

## Purpose

Placeholder of a new island node (`N*`) in Rust. It shows the package shape a
new node takes and is removed when the first real node lands.

## Inner-workings / Algorithms

The logic is `Sequencer`, a `si_nodes::StepNode`. The component binds it to a
timer and a publisher; it does no other work.

## Inputs / Outputs

### Output

| Name                        | Type                                           | Description                         |
| :-------------------------- | :--------------------------------------------- | :---------------------------------- |
| `/si/placeholder/heartbeat` | `autoware_safety_island_msgs/msg/HpcHeartbeat` | Sequence counter, every `period_ms` |

## Parameters

Declared once in [schema/si_placeholder_rust.schema.json](schema/si_placeholder_rust.schema.json),
defaults in [config/si_placeholder_rust.param.yaml](config/si_placeholder_rust.param.yaml).

| Name        | Type    | Description                         | Default |
| :---------- | :------ | :---------------------------------- | :------ |
| `period_ms` | integer | Publication period of the heartbeat | 100     |

nano-ros 0.6.0-nros8 resolves the launch value but does not deliver it to
`ctx.param()` yet, so the node runs on its built-in default of 100 ms.
