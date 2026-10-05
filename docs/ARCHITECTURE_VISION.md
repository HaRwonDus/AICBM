# AICBM — AI City Builder in Minecraft

**AICBM** is an AI-driven city construction platform built on top of Minecraft and designed primarily as a **QA and stress-testing workload for AI/ML infrastructure**.

The project combines autonomous agents, LLM inference, structured Minecraft world interaction, dataset generation, continuous learning, model fine-tuning, evaluation, and infrastructure monitoring in a single long-running workload.

The main objective is to create an autonomous AI system capable of planning, constructing, evaluating, and improving large-scale cities inside Minecraft while simultaneously generating realistic workloads on AI infrastructure.

---

# Project Goals

AICBM has two primary goals.

## 1. Autonomous City Building

The AI agent should be able to:

* analyze the Minecraft environment;
* understand construction goals;
* generate high-level city plans;
* decompose plans into construction tasks;
* build roads, districts, buildings, parks, and infrastructure;
* reuse previously learned construction skills;
* import new building templates;
* adapt new templates into reusable construction patterns;
* evaluate completed structures;
* learn from successful and failed construction attempts.

The long-term goal is to allow the system to construct large-scale environments such as:

> **A Minecraft representation of Almaty generated and built by autonomous AI agents.**

---

## 2. QA and Infrastructure Stress Testing

AICBM is also intended to operate as a realistic AI infrastructure benchmark.

Instead of running only synthetic GPU stress tests, the platform generates workloads through real AI tasks:

* LLM inference;
* agent orchestration;
* concurrent AI agents;
* QLoRA fine-tuning;
* dataset generation;
* model evaluation;
* Minecraft simulation;
* networking;
* persistent storage;
* telemetry;
* long-running workloads.

This allows AICBM to test AI/ML virtual machines and GPU infrastructure under complex mixed workloads.

---

# Architecture

```text
                       AICBM
              AI City Builder in Minecraft

┌─────────────────────────────────────────────────────────┐
│                    Rust Backend                         │
│                                                         │
│  ┌───────────────┐        ┌─────────────────────────┐   │
│  │ API Gateway   │        │ Agent Orchestrator      │   │
│  │ Axum / Tonic  │───────►│ Tokio                   │   │
│  └───────────────┘        └────────────┬────────────┘   │
│                                       │                 │
│          ┌────────────────────────────┼───────────┐     │
│          │                            │           │     │
│          ▼                            ▼           ▼     │
│    ┌───────────┐              ┌───────────┐ ┌────────┐ │
│    │ Planner   │              │World Model│ │ Skills │ │
│    └─────┬─────┘              └───────────┘ └───┬────┘ │
│          │                                       │      │
│          ▼                                       ▼      │
│    ┌─────────────┐                       ┌─────────────┐ │
│    │Build Engine │                       │ Template    │ │
│    └─────┬───────┘                       │ Registry    │ │
│          │                               └─────┬───────┘ │
│          ▼                                     │         │
│    ┌─────────────┐                             ▼         │
│    │Action Engine│                      ┌──────────────┐ │
│    └─────┬───────┘                      │Dataset       │ │
│          │                              │Builder       │ │
│          │                              └──────┬───────┘ │
│          │                                     │         │
│          │                              ┌──────▼───────┐ │
│          │                              │Training      │ │
│          │                              │Manager       │ │
│          │                              └──────┬───────┘ │
│          │                                     │         │
│          │                              ┌──────▼───────┐ │
│          │                              │Model Registry│ │
│          │                              └──────────────┘ │
└──────────┼───────────────────────────────────────────────┘
           │
         gRPC
           │
           ▼
┌─────────────────────────────┐
│ Java Minecraft Bridge       │
│ Paper Plugin                │
│                             │
│ World Sensor                │
│ Entity Sensor               │
│ Inventory Sensor            │
│ Event Listener              │
│ Action Executor             │
└──────────────┬──────────────┘
               │
               ▼
        Minecraft Server
```

---

# Technology Stack

## Backend

The backend is implemented entirely in **Rust**.

```text
Language            Rust
Async runtime       Tokio
HTTP API            Axum
RPC                 Tonic
Serialization       Serde
RPC format          Protocol Buffers
Database            PostgreSQL
Database driver     SQLx
Cache               Redis
Vector database     Qdrant
Metrics             Prometheus
Tracing             tracing
Parallel workloads  Rayon
```

Rust is responsible for:

* agent orchestration;
* world state management;
* task scheduling;
* city planning;
* construction planning;
* action execution coordination;
* skill management;
* blueprint processing;
* dataset generation;
* training job management;
* model lifecycle;
* QA scenarios;
* telemetry;
* infrastructure coordination.

---

# Minecraft Integration

Minecraft integration is implemented as a lightweight **Java Paper plugin**.

Java is not responsible for AI planning or backend business logic.

Its purpose is to expose Minecraft as a structured simulation environment.

```text
Rust Backend
     │
     │ gRPC
     ▼
Java Paper Bridge
     │
     ▼
Minecraft Server
```

The bridge provides:

* world state;
* nearby blocks;
* entities;
* inventory;
* Minecraft events;
* agent positions;
* action results;
* server performance statistics.

Supported actions may include:

```text
MoveTo
LookAt
PlaceBlock
BreakBlock
Interact
Craft
SelectItem
UseItem
ExecuteBuildStep
```

---

# Structured Environment Access

The first version of AICBM does not depend on screenshot capture or computer vision for basic Minecraft interaction.

Instead of:

```text
Minecraft
   ↓
Screenshot
   ↓
Webhook
   ↓
VLM / CV
   ↓
Agent
```

AICBM uses:

```text
Minecraft
   ↓
Java Bridge
   ↓
Structured Observation
   ↓
Rust Agent
```

Computer vision can later be added as an additional observation mode for dedicated VLM benchmarks.

---

# Agent Architecture

```text
Observation
     │
     ▼
World Model
     │
     ▼
Goal Manager
     │
     ▼
AI Planner
     │
     ▼
Build Planner
     │
     ▼
Action Engine
     │
     ▼
Minecraft Bridge
     │
     ▼
Minecraft
```

The LLM should not generate every individual block placement.

Instead, it generates hierarchical plans.

```text
Build Residential District
│
├── Generate road layout
├── Prepare terrain
├── Build apartment block A
├── Build apartment block B
├── Build apartment block C
├── Build parking area
├── Build park
└── Install lighting
```

The Rust Build Engine converts these goals into deterministic construction operations.

---

# World Model

AICBM maintains an internal representation of the Minecraft environment.

The World Model stores:

* terrain;
* blocks;
* agents;
* buildings;
* roads;
* zones;
* entities;
* available resources;
* construction progress;
* completed structures;
* known templates.

Communication should primarily use deltas.

```text
Initial World Snapshot
        ↓
World Delta
        ↓
World Delta
        ↓
World Delta
```

Example:

```text
BLOCK_PLACED 102 64 -225 minecraft:stone_bricks
BLOCK_BROKEN 103 64 -225 minecraft:dirt
AGENT_MOVED bot01 105.3 64 -220.8
```

---

# Blueprint System

Buildings are represented as reusable blueprints.

```rust
struct Blueprint {
    id: Uuid,
    name: String,
    category: BuildingCategory,
    dimensions: Dimensions,
    components: Vec<Component>,
    materials: Vec<MaterialRequirement>,
    constraints: Vec<Constraint>,
}
```

A blueprint may contain:

```text
Blueprint
├── Metadata
├── Dimensions
├── Foundation
├── Floors
├── Walls
├── Windows
├── Doors
├── Roof
├── Materials
├── Decorations
└── Constraints
```

---

# Template Learning

New construction templates can be introduced without immediately retraining the LLM.

```text
New Template
     ↓
Template Parser
     ↓
Blueprint Representation
     ↓
Structural Validation
     ↓
Template Registry
     ↓
Build Engine
```

Example:

```text
template_almaty_residential_9f.json
```

Once validated, the building becomes immediately available to the planning and construction system.

---

# Skill System

Repeated successful construction patterns can be converted into reusable skills.

Examples:

```text
build_wall
build_floor
build_window_grid
build_road
build_intersection
build_roof
build_highrise
build_residential_block
terraform
create_park
install_street_lighting
```

Learning pipeline:

```text
Construction
     ↓
Experience
     ↓
Evaluation
     ↓
Successful Pattern
     ↓
Skill Extraction
     ↓
Skill Registry
```

This prevents the LLM from repeatedly solving low-level deterministic construction problems.

---

# Learning Engine

The Learning Engine contains:

```text
Experience Collector
Dataset Builder
Skill Extractor
Template Learner
Training Manager
Evaluator
Model Registry
Deployment Manager
```

---

# Continuous Learning Strategy

AICBM uses a hybrid learning strategy.

```text
New Knowledge
     │
     ├── Blueprint / Template Memory
     │
     ├── Skill Memory
     │
     └── Model Fine-Tuning
```

Not every new building should trigger model training.

New information should first enter the template and skill systems.

Model fine-tuning occurs only after enough validated training data has accumulated.

---

# Fine-Tuning Method

The primary fine-tuning method is:

> **Supervised Fine-Tuning using QLoRA.**

The model itself remains mostly frozen.

Only low-rank adapter matrices are trained.

The base model weights are stored in low precision, typically **4-bit NF4 quantization**, significantly reducing VRAM requirements.

The training pipeline is:

```text
Validated Experiences
        ↓
Dataset Builder
        ↓
SFT Dataset
        ↓
QLoRA
        ↓
Candidate Adapter
        ↓
Evaluation
        ↓
Model Registry
```

---

# LoRA

For a standard neural network layer:

```text
y = Wx
```

LoRA does not directly train the original weight matrix `W`.

Instead:

```text
W' = W + ΔW
```

where:

```text
ΔW = BA
```

and:

```text
A ∈ R^(r × k)
B ∈ R^(d × r)
```

where `r` is the LoRA rank and:

```text
r << min(d, k)
```

The forward pass becomes:

```text
y = Wx + BAx
```

Usually LoRA scaling is applied:

```text
ΔW = (α / r) BA
```

Therefore:

```text
y = Wx + (α / r) BAx
```

where:

* `W` — frozen base model weights;
* `A`, `B` — trainable adapter matrices;
* `r` — adapter rank;
* `α` — scaling coefficient.

Only `A` and `B` are updated during training.

---

# QLoRA

QLoRA combines:

```text
Quantized LLM
+
LoRA adapters
```

The base matrix becomes quantized:

```text
W → Q(W)
```

The effective forward pass is approximately:

```text
y = dequant(Q(W))x + (α / r)BAx
```

During training:

```text
∂L / ∂W = 0
```

while:

```text
∂L / ∂A ≠ 0
∂L / ∂B ≠ 0
```

Therefore the large original model remains frozen while the much smaller adapter is optimized.

For AICBM this is important because training can run concurrently with:

```text
LLM inference
Minecraft agents
World processing
Dataset generation
Monitoring
```

on the same QA infrastructure.

---

# Supervised Fine-Tuning

Training samples are generated from validated construction trajectories.

Example:

```json
{
  "goal": "Build a nine-floor residential building",
  "environment": {
    "terrain": "flat",
    "available_area": [64, 64]
  },
  "plan": [
    "prepare foundation",
    "construct structural frame",
    "construct floors",
    "build facade",
    "install windows",
    "build roof"
  ],
  "result": {
    "success": true,
    "completion": 0.98
  }
}
```

The model learns:

```text
P(correct_plan | goal, world_state, constraints)
```

---

# SFT Loss Function

Training uses autoregressive cross-entropy.

For target token sequence:

```text
y1, y2, ..., yT
```

the loss is:

```text
L_SFT =
- Σ log Pθ(yt | y<t, x)
```

or normalized:

```text
L_SFT =
- (1 / T) Σ[t=1..T] log Pθ(yt | y<t, x)
```

where:

* `x` — input context;
* `yt` — expected token;
* `θ` — trainable LoRA parameters;
* `T` — number of supervised output tokens.

Prompt tokens can be masked so that loss is computed only for the desired agent output.

```text
L =
- 1/N Σ mt log Pθ(yt | y<t, x)
```

where:

```text
mt = 0 → ignore token
mt = 1 → include token
```

---

# Construction Experience

Each run produces an experience trajectory.

```text
τ = {
    Goal,
    Observations,
    Plans,
    Actions,
    Results,
    FinalWorldState
}
```

Conceptually:

```text
τ = (s0, a0, s1, a1, ..., sT)
```

where:

* `s` — world/agent state;
* `a` — action or planning decision.

---

# Construction Reward

A composite reward is calculated for completed tasks.

```text
R =
w₁S +
w₂B +
w₃C +
w₄E +
w₅T -
w₆F
```

where:

* `S` — structural correctness;
* `B` — blueprint similarity;
* `C` — completion ratio;
* `E` — resource efficiency;
* `T` — time efficiency;
* `F` — failure/error penalty.

Example weights:

```text
w₁ = 0.30
w₂ = 0.25
w₃ = 0.20
w₄ = 0.10
w₅ = 0.10
w₆ = 0.05
```

Therefore:

```text
R =
0.30S +
0.25B +
0.20C +
0.10E +
0.10T -
0.05F
```

All positive components should normally be normalized:

```text
0 ≤ metric ≤ 1
```

---

# Completion Ratio

A simple construction completion metric is:

```text
C =
correct_required_blocks
───────────────────────
total_required_blocks
```

Example:

```text
9700 required blocks correctly placed
10000 total blocks
```

gives:

```text
C = 9700 / 10000 = 0.97
```

---

# Blueprint Similarity

For voxel/block structures, several similarity metrics can be used.

A simple Jaccard / IoU metric is:

```text
IoU =
|P ∩ G|
─────────
|P ∪ G|
```

where:

* `P` — blocks placed by the agent;
* `G` — expected blueprint blocks.

A perfect reconstruction:

```text
IoU = 1.0
```

A more detailed system can compare separately:

```text
geometry
block type
orientation
position
functional components
```

---

# Block Precision and Recall

Precision:

```text
Precision =
correctly_placed_blocks
───────────────────────
all_placed_blocks
```

Recall:

```text
Recall =
correctly_placed_blocks
───────────────────────
required_blueprint_blocks
```

F1 score:

```text
F1 =
2 × Precision × Recall
──────────────────────
Precision + Recall
```

This helps distinguish:

* missing blocks;
* unnecessary blocks;
* incorrectly positioned blocks.

---

# Resource Efficiency

Resource efficiency may be calculated as:

```text
E_resource =
required_material_cost
──────────────────────
actual_material_cost
```

with the result capped to:

```text
0 ≤ E_resource ≤ 1
```

Waste can also be explicitly measured:

```text
Waste =
unused_or_wrong_blocks
──────────────────────
all_consumed_blocks
```

---

# Time Efficiency

Given reference execution time `T_ref` and actual time `T_actual`:

```text
E_time =
min(1, T_ref / T_actual)
```

Fast successful construction approaches:

```text
E_time → 1
```

while unnecessarily slow execution reduces the score.

---

# Failure Penalty

Failures can include:

```text
invalid action
collision
unreachable target
incorrect placement
repeated action
agent stall
timeout
bridge failure
```

Normalized failure rate:

```text
F =
failed_actions
──────────────
total_actions
```

---

# Dataset Admission

Not every trajectory should be used as a positive example.

A basic acceptance condition may be:

```text
R ≥ R_min
```

For example:

```text
R ≥ 0.85
```

Additional requirements may include:

```text
Completion ≥ 0.95
Blueprint IoU ≥ 0.90
Critical Errors = 0
```

Therefore:

```text
Accept(τ) =
(R ≥ R_min)
∧
(C ≥ C_min)
∧
(B ≥ B_min)
∧
(critical_errors = 0)
```

This prevents self-generated mistakes from contaminating the training dataset.

---

# Positive and Negative Experiences

AICBM should preserve both successful and failed trajectories.

```text
Experience Store
│
├── Positive
├── Negative
└── Ambiguous
```

Positive examples are used primarily for SFT.

Negative examples can later be used to create preference datasets.

---

# Preference Learning

Once AICBM has enough comparable trajectories, an optional second training stage can use **Direct Preference Optimization — DPO**.

For the same goal:

```text
x = construction request
```

we create:

```text
y+ = preferred plan
y- = inferior plan
```

Example:

```text
Goal:
Build a residential block.

Plan A:
98% completion
0 structural errors
12 minutes

Plan B:
87% completion
14 structural errors
19 minutes
```

The dataset records:

```text
Plan A > Plan B
```

---

# DPO Objective

The DPO loss can be expressed as:

```text
L_DPO =
-log σ(
β [
    log πθ(y+ | x)
    - log πref(y+ | x)
    - log πθ(y- | x)
    + log πref(y- | x)
]
)
```

where:

* `πθ` — trainable model;
* `πref` — reference model;
* `y+` — preferred response;
* `y-` — rejected response;
* `β` — preference strength;
* `σ` — sigmoid function.

The objective encourages the model to increase the relative likelihood of better construction strategies.

---

# Training Strategy

The planned training stages are:

```text
Stage 1
Base Instruction Model
        ↓
Stage 2
QLoRA + SFT
        ↓
AICBM-SFT Model
        ↓
Stage 3
Preference Dataset
        ↓
QLoRA + DPO
        ↓
AICBM-DPO Model
```

The initial implementation should stop at **QLoRA + SFT**.

DPO should be introduced only after enough reliable preference pairs have been collected.

---

# Why Not Continuous Online Fine-Tuning

The system should not perform gradient updates after every construction run.

That approach risks:

* catastrophic forgetting;
* unstable behavior;
* dataset contamination;
* reinforcement of agent mistakes;
* excessive GPU usage;
* difficult reproducibility.

Instead:

```text
Experience
     ↓
Buffer
     ↓
Validation
     ↓
Dataset Version
     ↓
Training Job
```

Training therefore operates in controlled batches.

---

# Training Trigger

Fine-tuning can be triggered when one or more thresholds are reached.

Example:

```text
validated_samples ≥ N_min
```

or:

```text
new_skills ≥ K
```

or:

```text
new_template_categories ≥ M
```

A practical initial condition may be:

```text
validated_samples >= 5000
```

rather than retraining after every single construction.

---

# Dataset Versioning

Datasets should be immutable and versioned.

Example:

```text
aicbm-dataset-v0.1
aicbm-dataset-v0.2
aicbm-dataset-v1.0
```

Each dataset version should preserve:

```text
source trajectories
template versions
agent version
validation rules
reward configuration
creation timestamp
statistics
```

---

# Model Evaluation

Every candidate model must be tested before deployment.

Suppose:

```text
M_old = current production model
M_new = newly trained candidate
```

For benchmark score:

```text
Score(M) =
(1 / N) Σ Ri
```

where `Ri` is the reward for benchmark scenario `i`.

The candidate should not automatically replace the active model.

A simple promotion rule may be:

```text
Score(M_new) > Score(M_old)
```

with additional hard constraints.

For example:

```text
Completion_new >= Completion_old
CriticalFailures_new <= CriticalFailures_old
P95Latency_new <= latency_limit
```

---

# Regression Detection

Model improvement must be checked across different construction categories.

Example benchmark suite:

```text
small_house
apartment_block
highrise
road_network
park
intersection
district
mixed_city
```

A model should not be promoted merely because it became better at one category while severely degrading another.

---

# Model Registry

Every trained model or adapter is versioned.

```text
aicbm-base-v1
aicbm-sft-v1
aicbm-sft-v2
aicbm-dpo-v1
```

Each version stores:

* base model;
* LoRA adapter;
* dataset version;
* training method;
* hyperparameters;
* LoRA rank;
* LoRA alpha;
* evaluation metrics;
* benchmark results;
* deployment status.

---

# QA Workloads

AICBM can create progressively heavier workloads.

## Scenario 1

```text
1 Minecraft Agent
+
LLM Inference
```

## Scenario 2

```text
8 Minecraft Agents
+
LLM Inference
+
World Model
```

## Scenario 3

```text
32 Minecraft Agents
+
Continuous Inference
+
City Planning
+
Dataset Generation
```

## Scenario 4

```text
Multiple Minecraft Agents
+
LLM Inference
+
QLoRA Training
+
Dataset Generation
+
Model Evaluation
+
Prometheus
+
Grafana
+
MLflow
```

This creates simultaneous pressure on:

```text
GPU
VRAM
CPU
RAM
Disk I/O
Network
Database
Scheduler
Inference runtime
Training runtime
```

---

# QA Metrics

## GPU

```text
GPU utilization
VRAM usage
GPU temperature
GPU power
Tensor/Core utilization
Inference throughput
Training throughput
```

## CPU

```text
CPU utilization
Load average
Thread utilization
Context switching
```

## Memory

```text
RAM usage
Swap usage
Memory pressure
Allocation rate
```

## AI

```text
Tokens/sec
Time to first token
Inference latency
Batch throughput
Training loss
Training throughput
Model load time
```

## Agent

```text
Active agents
Actions/sec
Successful actions
Failed actions
Agent stalls
Planning latency
Construction throughput
```

## Minecraft

```text
TPS
MSPT
Blocks placed/sec
Blocks broken/sec
Loaded chunks
Entities
Server memory
```

## Network

```text
gRPC RTT
Throughput
Dropped connections
Reconnects
Queue depth
```

---

# Monitoring

Monitoring stack:

```text
Prometheus
Grafana
MLflow
```

Prometheus collects infrastructure and application metrics.

Grafana provides operational dashboards.

MLflow tracks:

* experiments;
* fine-tuning runs;
* datasets;
* model versions;
* hyperparameters;
* training metrics;
* evaluation results.

---

# Repository Structure

```text
aicbm/
│
├── Cargo.toml
├── README.md
│
├── apps/
│   ├── aicbm-server/
│   ├── aicbm-worker/
│   └── aicbm-cli/
│
├── crates/
│   ├── api/
│   ├── protocol/
│   ├── agent-runtime/
│   ├── agent-manager/
│   ├── planner/
│   ├── world-model/
│   ├── build-engine/
│   ├── action-engine/
│   ├── blueprint/
│   ├── template-registry/
│   ├── skill-engine/
│   ├── skill-registry/
│   ├── memory/
│   ├── dataset/
│   ├── training/
│   ├── evaluation/
│   ├── model-registry/
│   ├── inference/
│   ├── qa/
│   ├── telemetry/
│   ├── storage/
│   └── common/
│
├── minecraft/
│   └── aicbm-paper-bridge/
│
├── proto/
│   ├── minecraft.proto
│   ├── inference.proto
│   └── training.proto
│
├── templates/
├── scenarios/
├── datasets/
├── models/
├── configs/
├── scripts/
├── tests/
│
└── infrastructure/
    ├── docker/
    ├── prometheus/
    ├── grafana/
    └── mlflow/
```

---

# Communication Protocol

Communication between Rust and Minecraft uses:

```text
gRPC
+
Protocol Buffers
```

Bidirectional streaming is preferred.

### Minecraft → Rust

```text
WorldSnapshot
WorldDelta
AgentState
InventoryState
EntityState
GameEvent
ActionResult
MinecraftMetrics
```

### Rust → Minecraft

```text
MoveTo
LookAt
PlaceBlock
BreakBlock
Interact
Craft
SelectSlot
ExecuteBuildStep
ResetScenario
PauseAgent
ResumeAgent
```

---

# Design Principles

## Rust-first Backend

All backend orchestration and business logic is implemented in Rust.

Java exists only as the Minecraft environment adapter.

## Hierarchical Planning

LLMs generate strategic and semantic plans rather than millions of individual block operations.

## Deterministic Execution

Low-level construction should be handled by the Rust Build Engine and reusable skills.

## Structured Environment Access

Minecraft exposes structured world information through the Java bridge.

## Memory Before Training

New templates should first become structured knowledge.

Gradient-based fine-tuning is used only when it provides measurable value.

## QLoRA Fine-Tuning

Model adaptation uses low-rank adapters over a quantized frozen base model to minimize GPU memory usage.

## Validated Learning

Only evaluated trajectories enter training datasets.

## Evaluation Before Deployment

Candidate models must pass benchmark and regression tests before promotion.

## Reproducible QA

Every workload and training run must be reproducible from configuration.

---

# Development Priorities

```text
1. Rust Workspace
2. Protocol Buffers contracts
3. Java Paper Bridge
4. Rust ↔ Minecraft gRPC
5. Observation system
6. Action Engine
7. World Model
8. Blueprint format
9. Build Engine
10. Basic AI Planner
11. Experience Collector
12. Dataset Builder
13. Reward System
14. Multi-agent runtime
15. QA Scenario Runner
16. Prometheus Metrics
17. MLflow Integration
18. Skill Registry
19. Training Manager
20. QLoRA SFT
21. Model Evaluator
22. Model Registry
23. Continuous Learning
24. DPO preference training
```

---

# MVP

The initial MVP should be able to:

* connect the Rust backend to Minecraft;
* create an AI-controlled agent;
* receive structured world observations;
* send construction actions;
* load a blueprint;
* build a structure;
* track construction progress;
* calculate construction metrics;
* calculate reward;
* record trajectories;
* export dataset samples;
* expose Prometheus metrics.

Example:

```text
Input:

Build a small residential district.

Result:

- road;
- three buildings;
- street lighting;
- park.

AICBM records:

- plan;
- actions;
- world states;
- construction time;
- inference latency;
- GPU utilization;
- Minecraft TPS;
- completion score;
- blueprint similarity;
- reward.
```

---

# Long-Term Learning Loop

```text
Natural Language Goal
        ↓
Autonomous Planning
        ↓
City Master Plan
        ↓
District Planning
        ↓
Building Planning
        ↓
Multi-Agent Construction
        ↓
Evaluation
        ↓
Experience Collection
        ↓
Template / Skill Learning
        ↓
Dataset Generation
        ↓
QLoRA SFT
        ↓
Model Evaluation
        ↓
Optional DPO
        ↓
Improved Agent
```

---

# Long-Term Vision

Minecraft is the first simulation environment, not a permanent architectural dependency.

Future adapters may include:

```text
Minecraft
Minetest
Unity
Unreal Engine
Custom Simulation Environments
```

The AICBM core should remain independent of the underlying simulation environment.

---

# Project Status

**Status:** Early Development / Architecture Phase

Current focus:

```text
Rust backend architecture
Minecraft integration
Agent protocol
Blueprint system
QA workload design
Continuous learning
QLoRA training pipeline
```

---

# Project Purpose

AICBM is not intended to be only a Minecraft bot.

It is designed as:

> **A Rust-based autonomous agent and continuous-learning platform that uses Minecraft as a simulation environment and AI infrastructure QA workload.**

The platform combines:

```text
Autonomous Agents
+
Simulation
+
LLM Inference
+
QLoRA Fine-Tuning
+
Preference Learning
+
Dataset Generation
+
Model Evaluation
+
Infrastructure Monitoring
+
Continuous Improvement
```

into one scalable AI/ML workload.
