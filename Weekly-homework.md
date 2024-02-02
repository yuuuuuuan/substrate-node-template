# Week 1

要求：

编写存证模块的单元测试代码，包括：

* 创建存证的测试用例 
* 撤销存证的测试用例 
* 转移存证的测试用例

实现：

![test](/img/test.PNG)

# Week 2

要求：

跟着视频,完成Kitties的开发：

* Pallet编译成功 ·单元测试通过
* 加入kitties pallet到runtime中,node可以编译通过
* node节点可以正常启动

实现：

![test](/img/test2.PNG)

Improve unit test 改进单元测试：

* 单元测试可以获取event

* 比较event的值

实现：

![test](/img/test2-2.PNG)

# Week 3

要求：

跟着视频完成kitties pallet：

- 增加Currency, sale/buy方法后Pallet 可以编译 
- 增加新的测试用例 
- 修改runtime, node可以编译通过 -node节点可以启动

实现：

![test](/img/test3.PNG)

![run](/img/run3.PNG)

Runtime升级：

- Kitties pallet v2，将kitties name扩充到8个字节
- 完成migration代码 
- 验证从v0-v2, v1-v2的升级路径

实现：

![run](/img/run3-2.PNG)

# Week 4

要求：

- 请回答链上随机数（如前面Kitties示例中）与链下随机数的区别 
- 在Offchain Worker中，使用Offchain Indexing特性实现从链上向Offchain Storage中写入数据 
- 使用 js sdk 从浏览器frontend获取到前面写入Offchain Storage的数据 
- 设计一个场景实例（比如获取一个外部的价格信息），实现从OCW中向链上发起带签名负载的不签名交易，并在Runtime中正确处理

实现：

​	链上随机数（如前面Kitties示例中）与链下随机数的区别：

链上随机数和链下随机数的主要区别在于生成随机数的场所不同。链上随机数是指在区块链上生成随机数。这意味着随机数生成过程是透明的，所有节点都可以验证随机数的真实性和不可预测性。链上随机数通常用于需要公平性和不可预测性的场景。链下随机数是指在链下生成随机数，然后将随机数上传到区块链上。这意味着随机数生成过程不透明，只有生成随机数的人才知道具体的生成过程。链下随机数通常用于不需要高安全性或公平性的场景。

![run4](/img/run4.PNG)

![polka2](/img/polka2.PNG)
# Week 5

要求：

编程作业，需要完成以下要求并且提交代码链接：

- 自己完成并部署一个erc20的智能合约

# Week 6

要求：

- 为 proof of existence (poe) 模块的可调用函数 create_claim, revoke_claim, transfer_claim 添加 benchmark 用例，并且将 benchmark 运行的结果应用在可调用函数上； 
- 选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）； 
- （附加题）根据 Chain Spec，部署公开测试网络

实现：

![run](/img/run6.)
