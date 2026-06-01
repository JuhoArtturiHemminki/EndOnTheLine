# EndOnTheLine

An ultra-fast, high-throughput digital signal processing (DSP) stream processing engine written in Rust. This software leverages parallelized SIMD (Single Instruction, Multiple Data) vectorization and real-time adaptive thresholding filters to evaluate streaming voltage, high-frequency sensor readings, and digital audio waveforms directly at hardware line speed.

Architected specifically under the **EndOnTheLine** design patterns to enforce maximal CPU cache locality, prevent memory bus saturation, and eliminate standard runtime overhead across modern multi-core processor environments.

***

## Author & Proprietary Ownership
* **Sole Creator & Owner:** Juho Artturi Hemminki
* **Licensing & Access Requests:** projectflagcarrier@gmail.com

***

## Usage Notice
This software is proprietary and is not distributed under any open-source license. Any commercial use, redistribution, or modification requires a valid license from the author. For all licensing enquiries, please contact **projectflagcarrier@gmail.com**.

***

## System Architecture & Technical Breakthroughs

The core implementation addresses the classic computing bottleneck known as the "Memory Wall"—a scenario where the raw computational capability of modern CPU vector units vastly outpaces the data delivery capacity of the system memory bus. By fundamentally restructuring how streaming data is ingested, evaluated, and compacted, the system achieves unprecedented throughput metrics.

### Hardware-Level Vectorization
The engine utilizes the Rust `portable_simd` ecosystem to explicitly command the compiler to generate target-specific vector registers. Standard 32-bit floating-point arrays are ingested directly into 16-element lanes (`f32x16`). Rather than processing elements sequentially through traditional scalar execution pipelines, the system dispatches mathematical comparisons simultaneously across wide hardware vector paths. This unlocks deep instruction-level parallelism (ILP) and allows the chip to bypass the standard Arithmetic Logic Unit (ALU) overhead entirely.

### Multi-Core Work Distribution
Workload parallelization is managed via a thread-isolated pipeline architecture implemented alongside the `Rayon` data-parallelism framework. Bulk telemetry arrays are broken down into discrete, contiguous data blocks optimized for multi-threaded scheduling. The execution topology is designed to eliminate inter-thread lock contention, data races, and state synchronization delays. By assigning independently processable segments to every available physical and logical CPU core, the application scales linearly with the system's core count.

### Real-Time Adaptive Thresholding Filter
The detection barrier is not static; it dynamically adapts to the characteristics of the incoming signal via an Exponential Moving Average (EMA) algorithm. The implementation continuously calculates a localized signal bias by executing fast horizontal vector reductions directly inside the CPU register space. The calculated average updates the internal kThreshold register using predefined scaling factors designed to compile into quick bit-shifting operations or hardware-level fused multiply-add (FMA) instructions, entirely eliminating slow division instructions from the critical loop.

### High-Density Bit Compaction
To minimize memory footprint and optimize bus utilization, the evaluation results are heavily compacted. Instead of allocating full bytes or booleans for individual comparison flags—which would result in catastrophic memory bandwidth wasting—the engine packs 16 conditional true/false vector flags down into a single `u16` word value. This achieves a critical **87.5% reduction in output memory bus traffic**, which keeps the system from hitting the physical memory saturation limits of the host motherboard.

### Cache Alignment & Locality Enforcements
The inner processing loops are strictly designed to avoid cache-line bouncing and unwanted Translation Lookaside Buffer (TLB) misses. By processing streams in sizes matched to the L1 and L2 silicon cache boundaries of modern performance chips, data remains hot inside the processor itself. Memory pre-fetching is fully utilized, meaning the next required data block is already waiting in cache before the active loop finishes its current vector evaluation.

***

## Empirical Performance Benchmarks

When compiled under localized target profiling on modern high-performance system architectures, the engine registers processing speeds operating at the boundary of modern motherboard bus limits.

### Single-Thread Peak Execution
On a solitary CPU processing strand, the SIMD core achieves approximately **4.0 Gelem/s** (Giga-elements per second). This translates to roughly **16 GB/s** of raw stream ingestion on a single core, proving that the math loops are fully optimized and execution speed is purely bound by the clock cycle limits of a single execution unit.

### Multi-Thread Parallel Execution
When distributing the stream arrays across all available hardware processing cores, the architecture reaches up to **8.95 Gelem/s** (exceeding **35 GB/s** of active stream ingestion). This massive performance scaling saturates the parallel memory channels of the system, delivering true hardware line speed.

***

## Project Directory Architecture

The repository layout is organized into distinct, clean modules separating the core processing logic, configuration schemas, and telemetry analysis tools:

* **Cargo.toml:** The central build manifest, defining structural criteria, compilation flags, profile optimizations, and external library version boundaries.
* **Cargo.lock:** The sealed configuration state tracking exact dependency versions to guarantee deterministic and reproducible builds across environment deployments.
* **.gitignore:** Isolation rules configuring version control to completely exclude temporary build targets, OS väliaikaistiedostot, and benchmarking artifacts.
* **src/lib.rs:** The production-ready kirjastotiedosto containing the raw SIMD processing functions and parallel scheduling layers.
* **benches/dsp_perf.rs:** The dedicated Criterion micro-benchmarking file used to execute isolated, statistically sound performance profiles under simulated signal stress.

***

## Deployment, Compilation & Verification

To verify that the target system is unlocking the absolute physical performance limits of its underlying chip architecture, you must issue custom vector commands directly to the compiler. This ensures the compiler generates native instructions matching your hardware layout.

Execute the following deployment command sequence inside your terminal environment:

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly bench
```

### Reviewing Telemetry Reports
The Criterion harness will isolate your processing cores and evaluate the code across a 100-sample test window, outputting real-time timing parameters:

* **Execution Envelope:** Displays the precise nanosecond or microsecond duration required to process a unified data matrix exceeding one million points.
* **Calculated Throughput:** Yields the absolute calculation score measured in Giga-elements per second (`Gelem/s`), allowing direct comparison against the documented peak speeds of 4.0 Gelem/s (single-core) and 8.95 Gelem/s (multi-core).
* **Statistical Distribution:** Identifies system outliers caused by OS background interrupts or thermal throttling, ensuring the reported mean execution time is highly accurate and reproducible.
