# rust-monty-comparison

Compare the performance of Montgomery multiplication and mulmod from num-bigint crate:

|            | Intel Xeon E5-2699 v4 | Apple M1 |
| ---------- | --------------------- | -------- |
| num-bigint | 405 ns                | 460 ns   |
| monty      | 380 s                 | 264 ns   |

The result is used in the following paper:

```bib
@inproceedings{ICPADS21_XRSA,
 author = {Dang, Fan and Li, Lingkun and Chen, Jiajie},
 title = {{xRSA: Construct Larger Bits RSA on Low-Cost Devices}},
 year = {2021},
 publisher = {IEEE},
 address = {New York, NY, USA},
 booktitle = {{Proceedings of the 27th IEEE International Conference on Parallel and Distributed Systems}},
}
```

