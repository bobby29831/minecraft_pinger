# minecraft_pinger
This is essentially a rewrite in Rust, with some changes, of my Java Minecraft Server pinger,
[bobby29831/MC-Status](https://github.com/bobby29831/MC-Status).

## Example Execution and Responses

### Large Servers
```shell
~$ ./minecraft_pinger hypixel.net
```

```yaml
Server status: Online
Server version: Requires MC 1.8 / 1.18 (Software Unknown)
Players: (55025/200000)
```

### Regular Server
```shell
~$ ./minecraft_pinger mc.roanoke.quest
```

```yaml
Server status: Online
Server version: forge arclight 1.16.5 (Software Unknown)
Players: (8/20) [complacentsee, Culbinium, Defaultyyy, DragonAlbert, frankzzz, Hihoshi32, Houta, OneCentCoin]
```

### Including Port
```shell
~$ ./minecraft_pinger uticacraft.tk:25565
```

```yaml
Server status: Online
Server version: 1.18.2 (Paper)
Players: (1/100) [MiloNeedsHelp]
```

