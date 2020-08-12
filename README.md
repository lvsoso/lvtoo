# lvtoo

### build

```shell
cargo build
```



### "sort"命令

对传入的一串数字进行排序，使用方式

```shell
>./target/debug/lvtoo sort 5 7 2 3 5
[
    2,
    3,
    5,
    5,
    7,
]
```



### "echo"命令

启动一个“echo server”，使用方式如下：

```shell
>./target/debug/lvtoo echo 8090
Server listening on port 8090
```

