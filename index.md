---
title: "❌ Test Result 2024-09-07 07:16 UTC"
date: 2024-09-07T07:16:41.806563906+00:00
categories: test-report
excerpt_separator: <!--more-->
---


| | Total | Passed | Failed | Ignored | Filtered | Duration |
| --- | ----- | -------| ------ | ------- | -------- | -------- |
| ❌ | 2 | 1 | 1 | 0 | 0 | 0s |


**Git:** `git@github.com:subhankar-nitt/rust-contract-testing.git` @ `refs/heads/master`

    Commit: a7442605aa89659810e2e419da8f09ce94f26671
    Author: subhankar <sub.nitt@gmail.com>
    Date: Sat, 7 Sep 2024 01:38:30 -0530

        added files

<!--more-->

# Index

| Name | Result | Duration |
| ---- | ------ | -------- |
| [test_get_user](#test_get_user) | ✅ | 0s | 
| [test_provider](#test_provider) | ❌ | 0s | 


# Details

## ✅ test_get_user<a id="test_get_user"></a>

**Duration**: 0s

## ❌ test_provider<a id="test_provider"></a>

**Duration**: 0s

<details>

<summary>Test output</summary>

<pre>
thread &#x27;test_provider&#x27; panicked at tests/producer_test.rs:141:17:
Body Error Some([BodyMismatch { path: &quot;$&quot;, expected: Some(b&quot;{\&quot;comment\&quot;:\&quot;user added \&quot;,\&quot;id\&quot;:1,\&quot;user_name\&quot;:\&quot;subhankar\&quot;}&quot;), actual: Some(b&quot;{\&quot;comment\&quot;:\&quot;user added \&quot;,\&quot;id\&quot;:1,\&quot;user_name\&quot;:\&quot;ubhankar\&quot;}&quot;), mismatch: &quot;Expected body &#x27;{\&quot;comment\&quot;:\&quot;user added \&quot;,\&quot;id\&quot;:1,\&quot;user_name\&quot;:\&quot;subhankar\&quot;}&#x27; to match &#x27;{\&quot;comment\&quot;:\&quot;user added \&quot;,\&quot;id\&quot;:1,\&quot;user_name\&quot;:\&quot;ubhankar\&quot;}&#x27; using equality but did not match&quot; }])
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

</pre>

</details>
