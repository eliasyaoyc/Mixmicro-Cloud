# ES-Bully

A variant of the Bully consensus algorithm like es.

# What is Bully? 
Bully 是一个基于序号选主的算法即集群中节点 id 值最大的那个则会被选为主节点
有三种类型的消息:
* Election 消息（选举消息）：用于发起选举
* Alive 消息（响应消息）：对Election 消息的应答
* Victory 消息（宣布胜利消息）：竞选成功的节点向其他节点宣誓主权

选主流程：
1. 首先每个节点都有一个id，每个节点都会判断自己的id在集群中是否是最大的，如果是最大的，则会向其他节点发送 Victory ，进行宣誓主权
2. 如果自己并不是当前集群中节点最大的，则需要向比自己大的节点发送 Election 消息，并等待其他节点的回复
3. 在规定时间内，若没有收到任何节点的 Alive 消息，则会认为自己成为主节点，并向其他节点发送 Victory，宣誓自己的是老大，若收到了比自己大的节点恢复的 Alive 消息，
   则自己会等待 Victory 的通知
4. 若收到了比自己小的节点发送过来的Election 消息，则会回复个 Alive 消息过去，告诉它我比让大

[选主流程图](https://zhuanlan.zhihu.com/p/110015509)

优点
* 选举速度快，算法复杂度低，容易实现

缺点
* 每个节点必须知道其他存活节点的 id，需要额外的信息存储
* 若一个 id 值比当前集群中 master 大的节点加入或者故障恢复，则会出现新一轮选主，如果这台节点不稳定频繁退出，则会出现频繁选主

问题：
1. 如何防止过期消息干扰 -> term
2. Master 假死 
3. 如何防止脑裂 -> 过半

# What dose the es improve?
* 编号比较
>  * 比较 ClusterState 版本号，版本号越大优先级越高 
>  * 比较 节点ID，节点越小优先级越高
>  * 对于上面的 **『问题一』** ES 也是通过 ClusterState 中的版本号来充当term，每个发起重新选举(新增节点或者节点退出)，都会把 ClusterState 中的版本号+1，由 Master 节点发送给 Follower 节点进行同步 

* Master 假死
>如果一个节点检测发现 Master 节点宕机了，如果立即发生重新选举，而Master 节点只是假死则恢复过来之后会在进行一次重新选举，故 ES 是一旦有节点发现Master 节点宕机了，则会通知其他节点去检测 Master 节点
是否宕机，如果有 1/2 以上的节点认为没宕机，则不会发起重新选举  

* 脑裂问题
>什么是：由于网络分区，导致不同网络环境下同时出现了 Master 节点
> 
>ES 通过 quorum 算法来解决这个问题，通过配置 discovery.zen.minimum_master_nodes 来配置有多少个节点，也就是说当一个节点想成为 Master,则需要 >= 这个配置的节点响应才能成为 Master
假如发生了脑裂问题，ES 为了避免异常ClusterState 对集群产生影响，则把发送 ClusterState 做成了 2pc（send 和 commit），也就是说，先发送 send 指令，发 ClusterState 发送到所有的
Follower 节点但是 Follower 节点并不会直接使用，而是先会先判断 ClusterState 中的 version 然后在发送响应给 Master，Master 如果在(discovery.zen.commit_timeout) 没有收到
discovery.zen.minimum_master_nodes 个数的响应，那么 Master 节点就不会发送 commit指令，反之发送，并等待**所有**节点响应，discovery.zen.publish_timeout，如果发现任何一个节点
没有响应，则再次更新整个集群状态  

* Master 降级
>除了 Follower 节点来检测 Master 的节点的存活，Master 节点也会自检
如果 master 节点自己发生能连接到的其他节点数目小于 n/2+1，则master 自动降级为 Follower 重新选举
如果 master 节点ping其他节点，发现了其他的 master(自己网络分区了)，则比较当前master 中ClusterState 的 version 和自己的version，如果小于，则降级并主动加入当前 master 节点  

* 网络负载问题
>集群中的每个节点成员都会维护和其他所有成员的交互，整个集群维护的网络连接的总数n*(n-1),如果集群中的节点数目非常多，则网络连接数目也会非常多，网络负载会比较大
但是ES 节点数目往往比较少，所以这个问题 ES 不会产生什么影响，也可以通过 discovery.zen.ping.unicats.concurrent_connects 这个参数来限制单播的最大连接数目，默认是10  

# License

Licensed under of either of

* MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
