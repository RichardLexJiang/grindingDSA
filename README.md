# DSA-grinding

**Data Structure & Algorithm** practice, mostly just grinding [leetcode](http://leetcode.com)

> *due to historical reason, there is many Chinese comments in java code.*

## LeetCode Grinding

|     题号      |                           题名                           |   难度   |                                                  简介                                                  |                                               题解                                               |
|:-----------:|:------------------------------------------------------:|:------:|:----------------------------------------------------------------------------------------------------:|:----------------------------------------------------------------------------------------------:|
|      1      |                        Two Sum                         |  Easy  |                                         1.two pointer 2.Map                                          |       [java](java/src/leetCode/repository/No1.java)    [rust](rust/src/leetcode/no_1.rs)       |
|      2      |                    Add Two Numbers                     | Medium |                                           rugular problem                                            |       [java](java/src/leetCode/repository/No2.java)    [rust](rust/src/leetcode/no_2.rs)       |
|      3      |     Longest Substring Without Repeating Characters     | Medium |                                                                                                      |                               [rust](rust/src/leetcode/no_3.rs)                                |
|      5      |             Longest Palindromic Substring              |  Easy  |                                      1.DP 2.spread from center                                       |    [java](java/src/leetCode/repository/No5最长回文子串.java)    [rust](rust/src/leetcode/no_5.rs)    |
|      6      |                   Zigzag Conversion                    | Medium |                                                                                                      |                               [rust](rust/src/leetcode/no_6.rs)                                |
|      7      |                    Reverse Integer                     | Medium |                                                stack                                                 |                               [rust](rust/src/leetcode/no_7.rs)                                |
|      8      |                String to Integer (atoi)                | Medium |                                                string                                                |                               [rust](rust/src/leetcode/no_8.rs)                                |
|      9      |                     is palindrome                      |  Easy  |                                                 Easy                                                 |                               [rust](rust/src/leetcode/no_9.rs)                                |
|     10      |                        正则表达式匹配                         |  Hard  |                                              依旧普通的递归搜索题                                              |                         [java](java/src/leetCode/repository/No10.java)                         |
|     11      |               Container With Most Water                | Medium |                                  1.two pointer 2.spread from center                                  |   [java](java/src/leetCode/repository/No11盛最多水的容器.java)  [rust](rust/src/leetcode/no_11.rs)    |
|     12      |                    Integer to Roman                    | Medium |                                             divide + mod                                             |    [java](java/src/leetCode/repository/No12整数转罗马数字.java) [rust](rust/src/leetcode/no_12.rs)    |
|     13      |                    Roman to Integer                    |  Easy  |                                              just Easy                                               |   [java](java/src/leetCode/repository/No13罗马数字转整数.java)  [rust](rust/src/leetcode/no_13.rs)    |
|     14      |                 Longest Common Prefix                  |  Easy  |                                              just Easy                                               |                               [rust](rust/src/leetcode/no_14.rs)                               |
|     15      |                          3Sum                          | Medium | 1.fix one pointer and make two pointer close O(n^2) 2.fix two pointer and binary search O(n^2log(n)) |    [java](java/src/leetCode/repository/No15_3sum.java)  [rust](rust/src/leetcode/no_15.rs)     |
|     16      |                      3Sum Closest                      | Medium |                                         almost same with 15                                          | [java](java/src/leetCode/repository/No16最接近的三数之和.java)      [rust](rust/src/leetcode/no_16.rs) |
|     17      |         Letter Combinations of a Phone Number          | Medium |                                               Easy DFS                                               | [java](java/src/leetCode/repository/No17电话号码的字母组合.java)    [rust](rust/src/leetcode/no_17.rs)  |
|     18      |                          4Sum                          | Medium |                                          Similar to 15.3Sum                                          |    [java](java/src/leetCode/repository/No18四数之和.java)   [rust](rust/src/leetcode/no_18.rs)     |
|     19      |                      删除链表的倒数第N个节点                      | Medium |                                                 快慢指针                                                 |                   [java](java/src/leetCode/repository/No19删除链表的倒数第N个节点.java)                   |
|     20      |                         有效的括号                          |  Easy  |                                                数据结构-栈                                                |                      [java](java/src/leetCode/repository/No20有效的括号.java)                       |
|     21      |                        合并两个有序链表                        |  Easy  |                                               数据结构-链表                                                |                     [java](java/src/leetCode/repository/No21合并两个有序链表.java)                     |
|     22      |                          括号生成                          | Medium |                                                一般的搜索题                                                |               [java](java/src/leetCode/repository/No22GenerateParentheses.java)                |
|     24      |                       两两交换链表中的节点                       |  Easy  |                                                一般的递归                                                 |                    [java](java/src/leetCode/repository/No24两两交换链表中的节点.java)                    |
|     25      |                       K 个一组翻转链表                        |  Hard  |                                                 模拟题                                                  |               [java](java/src/leetCode/repository/No25ReverseNodesInKGroup.java)               |
|     28      |                    找出字符串中第一个匹配项的下标                     |  Easy  |                                               经典字符串匹配                                                |                 [java](java/src/leetCode/repository/No28找出字符串中第一个匹配项的下标.java)                  |
|     29      |                  Divide Two Integers                   | Medium |                                                                                                      |                               [rust](rust/src/leetcode/no_29.rs)                               |
|     32      |                         最长有效括号                         |  Hard  |                                            贪心/栈/动态规划 多种解法                                            |                      [java](java/src/leetCode/repository/No32最长有效括号.java)                      |
|     33      |                        搜索旋转排序数组                        | Medium |                                                 二分搜索                                                 |                     [java](java/src/leetCode/repository/No33搜索旋转排序数组.java)                     |
|     34      |                 在排序数组中查找元素的第一个和最后一个位置                  | Medium |                                            二分搜索,但要找最左和最右                                             |              [java](java/src/leetCode/repository/No34在排序数组中查找元素的第一个和最后一个位置.java)               |
|     42      |                          接雨水                           |  Hard  |                                              太久了,忘了内容了                                               |                [java](java/src/leetCode/repository/No42TrappingRainWater.java)                 |
|     45      |                         跳跃游戏II                         | Medium |                                            可将图画出来,发现就是最短路                                            |                      [java](java/src/leetCode/repository/No45跳跃游戏II.java)                      |
|     53      |                         最大子数组和                         |  Easy  |                                            dp思路或者可以用数学推导求                                            |                      [java](java/src/leetCode/repository/No53最大子数组和.java)                      |
|     54      |                          螺旋矩阵                          | Medium |                                   就是模拟,不过可用四个变量标记已遍历过的行和列来优化空间复杂度                                    |                       [java](java/src/leetCode/repository/No54螺旋矩阵.java)                       |
|     67      |                         二进制求和                          |  Easy  |                                         高精度数API可解,题解也有位运算法解                                          |                    [java](java/src/leetCode/repository/No67AddBinary.java)                     |
|     84      |                       柱状图中最大的矩形                        |  Hard  |                                             运用栈可以进一步优化.                                              |                         [java](java/src/leetCode/repository/No84.java)                         |
|     88      |                        合并两个有序数组                        |  Easy  |                                      Easy题,优化小点在从后往前遍历指针来节省空间.                                       |                     [java](java/src/leetCode/repository/No88合并两个有序数组.java)                     |
|     103     |                      二叉树的锯齿形层序遍历                       | Medium |                                              基础的层序遍历题.                                               |                   [java](java/src/leetCode/repository/No103二叉树的锯齿形层序遍历.java)                   |
|     137     |                      只出现一次的数字 II                       | Medium |  普通做法很Easy,[不使用额外空间实现](https://blog.csdn.net/jiangxiewei/article/details/82227451) 就略麻烦,可能需要逻辑代数知识   |                        [java](java/src/leetCode/repository/No137.java)                         |
|     146     |                        LRU 缓存机制                        | Medium |                                               模拟/数据结构                                                |                    [java](java/src/leetCode/repository/No146LRUCache.java)                     |
|     160     |                       No160相交链表                        | Medium |                                    我用求合法做到空间O(1),然而官方题解二的双指针也十分巧妙                                    |                      [java](java/src/leetCode/repository/No160相交链表.java)                       |
|     206     |                          反转链表                          |  Easy  |                                               基础数据结构操作                                               |                      [java](java/src/leetCode/repository/No206反转链表.java)                       |
|     209     |                        长度最小的子数组                        | Medium |                             滑动窗口法,可让左右区间滑动搜索实现O(n),也可用二分法针对每个左开区间寻找右闭区间.                             |                    [java](java/src/leetCode/repository/No209长度最小的子数组.java)                     |
|     210     |                         课程表II                          | Medium |                                           拓扑排序 + 深度/广度优先遍历                                           |                      [java](java/src/leetCode/repository/No210课程表II.java)                      |
|     240     |                        搜索二维矩阵II                        | Medium |                                                 找规律                                                  |                    [java](java/src/leetCode/repository/No240搜索二维矩阵II.java)                     |
|     297     |                      二叉树的序列化与反序列化                      |  Hard  |                                               树的遍历的运用.                                               |        [java](java/src/leetCode/repository/No297SerializeAndDeserializeBinaryTree.java)        |
|     315     |                     计算右侧小于当前元素的个数                      |  Hard  |                                         树状数组 + 离散化,注意下数据有重复                                          |         [java](java/src/leetCode/repository/No315CountOfSmallerNumbersAfterSelf.java)          |
|     416     |                         分割等和子集                         | Medium |                             01背包题,由于"物品"没有"价值",所以结果值可以只用true false代表可达性                              |             [java](java/src/leetCode/repository/No416PartitionEqualSubsetSum.java)             |
|     445     |                         两数相加II                         | Medium |                                                  栈                                                   |                     [java](java/src/leetCode/repository/No445两数相加II.java)                      |
|     503     |                Next Greater Element II                 | Medium |                                                Stack                                                 |                              [rust](rust/src/leetcode/no_503.rs)                               |
|     528     |                        按权重随机选择                         | Medium |                                               随机算法方案?                                                |                     [java](java/src/leetCode/repository/No528按权重随机选择.java)                     |
|     543     |                         二叉树的直径                         | Medium |                                   分治法,算出每个节点的左最大深度和右最大深度,比较并返回更大的                                    |                     [java](java/src/leetCode/repository/No543二叉树的直径.java)                      |
|     739     |                          每日温度                          | Medium |                                               单调栈 或 贪心                                               |                [java](java/src/leetCode/repository/No739DailyTemperatures.java)                |
|     749     |                          隔离病毒                          |  Hard  |                                             搜索、模拟、(并查集)                                              |                        [java](java/src/leetCode/repository/No749.java)                         |
|     856     |                         括号的分数                          | Medium |                                           1.暴力 2.栈 3.数学思维                                            |                      [java](java/src/leetCode/repository/No856括号的分数.java)                      |
|     862     |                     和至少为 K 的最短子数组                      |  Hard  |                                       找定义,找优化点,比较过程中更新比较集来实现优化                                       |         [java](java/src/leetCode/repository/No862shortestSubarrayWithSumAtLeastK.java)         |
|     912     |                          排序数组                          | Medium |                                           基础排序实现,这里我用了归并.                                            |                   [java](java/src/leetCode/repository/No912SortAnArrya.java)                   |
|     945     |                       使数组唯一的最小增量                       | Medium |                                               每日推荐里的水题                                               |        [java](java/src/leetCode/repository/No945MinimumIncrementToMakeArrayUnique.java)        |
|     990     |                       等式方程的可满足性                        |  Easy  |                                            模板题,可以测试自己写的模板                                            |        [java](java/src/leetCode/repository/No990SatisfiabilityOfEqualityEquations.java)        |
|    1014     |                         最佳观光组合                         |  Easy  |                                                  贪心                                                  |              [java](java/src/leetCode/repository/No1014BestSightseeingPair.java)               |
|    1143     |                        最长公共子序列                         | Medium |                                                二维动态规划                                                |                    [java](java/src/leetCode/repository/No1143最长公共子序列.java)                     |
|    1496     |                        判断路径是否相交                        |  Easy  |                                                 签到题                                                  |                  [java](java/src/leetCode/repository/No1496PathCrossing.java)                  |
|    1497     |                       仅含 1 的子串数                        | Medium |                                          找性质,找到两个重要性质就很好借.                                           |        [java](java/src/leetCode/repository/No1497CheckIfArrayPairsAreDivisibleByK.java)        |
|    1512     |                         好数对的数目                         |  Easy  |                                                 签到题                                                  |               [java](java/src/leetCode/repository/No1512NumberOfGoodPairs.java)                |
|    1513     |                       仅含 1 的子串数                        | Medium |                                                等差数列求和                                                |          [java](java/src/leetCode/repository/No1513NumberOfSubstringsWithOnly1s.java)          |
|    1514     |                        概率最大的路径                         | Medium |                                               Dijkstra                                               |           [java](java/src/leetCode/repository/No1514PathWithMaximumProbability.java)           |
|    1515     |                       服务中心的最佳位置                        |  Hard  |                                              证明凸函数,三分夹逼                                              |         [java](java/src/leetCode/repository/No1515BestPositionForAServiceCentre.java)          |
|    1518     |                          换酒问题                          |  Easy  |                                                 签到题                                                  |                  [java](java/src/leetCode/repository/No1518WaterBottles.java)                  |
|    1519     |                      子树中标签相同的节点数                       | Medium |                                               邻接表,DFS                                                |   [java](java/src/leetCode/repository/No1519NumberOfNodesInTheSubTreeWithTheSameLabel.java)    |
|    1521     |                      找到最接近目标值的函数值                      |  Hard  |                                          利用与运算性质以及结果集数据量限制                                           | [java](java/src/leetCode/repository/No1521FindAValueOfAMysteriousFunctionClosestToTarget.java) |
|    1544     |                         整理字符串                          |  Easy  |                                            1.模拟 2.栈 3.递归                                             |               [java](java/src/leetCode/repository/No1544MakeTheStringGreat.java)               |
|    1545     |                  找出第 N 个二进制字符串中的第 K 位                  | Medium |                                               溯源,可递归.                                                |          [java](java/src/leetCode/repository/No1545FindKthBitInNthBinaryString.java)           |
|    1599     |                       经营摩天轮的最大利润                       | Medium |                                                  模拟                                                  |                        [java](java/src/leetCode/repository/No1599.java)                        |
|    1606     |                      找到处理最多请求的服务器                      |  Hard  |                              以模拟的前提下通过busy优先队列和available红黑树减少全服务器的遍历操作.                              |                  [java](java/src/leetCode/repository/No1606找到处理最多请求的服务器.java)                  |
|    1775     |                    通过最少操作次数使数组的和相等                     | Medium |                                                  模拟                                                  |                        [java](java/src/leetCode/repository/No1775.java)                        |
|    1969     |                      数组元素的最小非零乘积                       | Medium |                                           寻找数学规律吧，另外别忘了快速幂                                           |    [java](java/src/leetCode/repository/No1969MinimumNonZeroProductOfTheArrayElements.java)     |
|    2475     |                      数组中不等三元组的数目                       |  Easy  |                                             1.暴力 2.数学+哈希                                             |                  [java](java/src/leetCode/repository/No2475数组中不等三元组的数目.java)                   |
|    3169     |                        无需开会的工作日                        | Medium |                                                 1.模拟                                                 |                        [java](java/src/leetCode/repository/No3169.java)                        |
|    3175     |                   找到连续赢 K 场比赛的第一位玩家                    | Medium |                                              1.模拟 2.找规律                                              |                        [java](java/src/leetCode/repository/No3175.java)                        |
|    3254     |                    长度为 K 的子数组的能量值 I                    | Medium |                                                                                                      |                              [rust](rust/src/leetcode/no_3255.rs)                              |
|    3255     |         Find the Power of K-Size Subarrays II          | Medium |                                                                                                      |                              [rust](rust/src/leetcode/no_3255.rs)                              |
|    3256     |       Maximum Value Sum by Placing Three Rooks I       | Medium |                                            same with 3257                                            |                              [rust](rust/src/leetcode/no_3257.rs)                              |
|    3257     |      Maximum Value Sum by Placing Three Rooks II       |  Hard  |                                                                                                      |                              [rust](rust/src/leetcode/no_3257.rs)                              |
|    3264     | Final Array State After K Multiplication Operations I  |  Easy  |                                                 Easy                                                 |                              [rust](rust/src/leetcode/no_3264.rs)                              |
|    3265     |               Count Almost Equal Pairs I               | Medium |                                                brute                                                 |                              [rust](rust/src/leetcode/no_3265.rs)                              |
|    3266     | Final Array State After K Multiplication Operations II |  Hard  |                                       quick_pow + heap + greed                                       |                              [rust](rust/src/leetcode/no_3266.rs)                              |
|    3270     |                         求出数字答案                         |  Easy  |                                                Easy暴力                                                |                        [java](java/src/leetCode/repository/No3270.java)                        |
|    3271     |                        哈希分割字符串                         | Medium |                                                Easy模拟                                                |                        [java](java/src/leetCode/repository/No3271.java)                        |
|    3272     |                        统计好整数的数目                        |  Hard  |                                              枚举 + 排列组合                                               |                        [java](java/src/leetCode/repository/No3272.java)                        |
|    3273     |                     对 Bob 造成的最少伤害                      |  Hard  |                                                 数学分析                                                 |                        [java](java/src/leetCode/repository/No3273.java)                        |
|    3274     |                      检查棋盘方格颜色是否相同                      |  Easy  |                                                Easy水题                                                |                        [java](java/src/leetCode/repository/No3274.java)                        |
|    3289     |                       数字小镇中的捣蛋鬼                        |  Easy  |                                                Easy水题                                                |                        [java](java/src/leetCode/repository/No3289.java)                        |
|    3290     |                         最高乘法得分                         |  Easy  |                                                EasyDP                                                |                        [java](java/src/leetCode/repository/No3290.java)                        |
| 3633 & 3635 |     Earliest Finish Time for Land and Water Rides      | Medium |                                             sort and ...                                             |                              [rust](rust/src/leetcode/no_3635.rs)                              |
|    3634     |           Minimum Removals to Balance Array            | Medium |                                          sort , two pointer                                          |                              [rust](rust/src/leetcode/no_3634.rs)                              |
|  面试题16.18   |                          模式匹配                          | Medium |                                             递归,枚举,剪枝,边界                                              |            [java](java/src/leetCode/repository/Mst_16_18_PatternMatchingLcci.java)             |
|  面试题17.16   |                          按摩师                           |  Easy  |                                             基本DP题,可当入门题                                              |              [java](java/src/leetCode/repository/Mst_17_16_TheMasseuseLcci.java)               |
|    面试题51    |                        数组中的逆序对                         |  Hard  |                                              树状数组 或 归并法                                              |        [java](java/src/leetCode/repository/Mst_51_shu_zu_zhong_de_ni_xu_dui_lcof.java)         |
|   LCP 33    |                           蓄水                           |  Easy  |                                           1.暴力枚举 2.优先队列+贪心                                           |                      [java](java/src/leetCode/repository/LCP_33_蓄水.java)                       |
|   LCP 80    |                         生物进化录                          | Medium |                                              1.遍历树 2.规律                                              |                        [java](java/src/leetCode/repository/LCP_80.java)                        |

## DSA Genre

### BinaryTree

|      题号      |     题名      |   难度   |               简介                |                             题解                             |
|:------------:|:-----------:|:------:|:-------------------------------:|:----------------------------------------------------------:|
| leetcode 103 | 二叉树的锯齿形层序遍历 | Medium |            基础的层序遍历题             | [java](java/src/leetCode/repository/No103二叉树的锯齿形层序遍历.java) |
| leetcode 543 |   二叉树的直径    |  Easy  | 分治法,算出每个节点的左最大深度和右最大深度,比较并返回更大的 |   [java](java/src/leetCode/repository/No543二叉树的直径.java)    |

### Stack

|      题号       |           题名            |   难度   |       简介        |                                 题解                                 |
|:-------------:|:-----------------------:|:------:|:---------------:|:------------------------------------------------------------------:|
|  leetcode 32  |         最长有效括号          |  Hard  | 贪心/栈/动态规划 多种解法  |        [java](java/src/leetCode/repository/No32最长有效括号.java)        |
| leetcode 445  |         两数相加II          | Medium |        栈        |       [java](java/src/leetCode/repository/No445两数相加II.java)        |
| leetcode 503  | Next Greater Element II | Medium |      Stack      |                [rust](rust/src/leetcode/no_503.rs)                 |
| leetcode 739  |          每日温度           | Medium |    单调栈 或 贪心     |  [java](java/src/leetCode/repository/No739DailyTemperatures.java)  |
| leetcode 856  |          括号的分数          | Medium | 1.暴力 2.栈 3.数学思维 |        [java](java/src/leetCode/repository/No856括号的分数.java)        |
| leetcode 1544 |          整理字符串          |  Easy  |  1.模拟 2.栈 3.递归  | [java](java/src/leetCode/repository/No1544MakeTheStringGreat.java) |

### DFS & BFS

|      题号      |          题名           |   难度   |                      简介                      |                                        题解                                        |
|:------------:|:---------------------:|:------:|:--------------------------------------------:|:--------------------------------------------------------------------------------:|
| leetcode 10  |        正则表达式匹配        |  Hard  |                  依旧普通的递归搜索题                  |                  [java](java/src/leetCode/repository/No10.java)                  |
| leetcode 22  |         括号生成          |  Easy  |                      ez                      |        [java](java/src/leetCode/repository/No22GenerateParentheses.java)         |
| leetcode 33  |       搜索旋转排序数组        | Medium |                     二分搜索                     |              [java](java/src/leetCode/repository/No33搜索旋转排序数组.java)              |
| leetcode 34  | 在排序数组中查找元素的第一个和最后一个位置 | Medium |                二分搜索,但要找最左和最右                 |       [java](java/src/leetCode/repository/No34在排序数组中查找元素的第一个和最后一个位置.java)        |
| leetcode 209 |       长度最小的子数组        | Medium | 滑动窗口法,可让左右区间滑动搜索实现O(n),也可用二分法针对每个左开区间寻找右闭区间. |             [java](java/src/leetCode/repository/No209长度最小的子数组.java)              |
| leetcode 297 |     二叉树的序列化与反序列化      |  Hard  |                   树遍历也是搜索                    | [java](java/src/leetCode/repository/No297SerializeAndDeserializeBinaryTree.java) |

### DP

|        题号         |    题名     |   难度   |                     简介                      |                                            题解                                            |
|:-----------------:|:---------:|:------:|:-------------------------------------------:|:----------------------------------------------------------------------------------------:|
|    leetcode 5     |  最长回文子串   |  Easy  |                  找准状态转移逻辑                   |  [java](java/src/leetCode/repository/No5最长回文子串.java)  [rust](rust/src/leetcode/no_5.rs)  |
|    leetcode 53    |   最大子序和   |  Easy  |               dp思路或者可以用数学推导求                |                   [java](java/src/leetCode/repository/No53最大子数组和.java)                   |
| leetcode 面试题17.16 |    按摩师    |  Easy  |         尝试从DFS->尾递归->DP的一个推断流程,注释里有         |           [java](java/src/leetCode/repository/Mst_17_16_TheMasseuseLcci.java)            |
|  leetcode 面试题46   | 把数字翻译成字符串 |  Easy  |                   也可以用搜索                    | [java](java/src/leetCode/repository/Mst_46_ba_shu_zi_fan_yi_cheng_zi_fu_chuan_lcof.java) |
|   leetcode 416    |  分割等和子集   | Medium | 01背包题,由于"物品"没有"价值",所以结果值可以只用true false代表可达性 |          [java](java/src/leetCode/repository/No416PartitionEqualSubsetSum.java)          |
|   leetcode 1143   |  最长公共子序列  | Medium |                   二维动态规划                    |                 [java](java/src/leetCode/repository/No1143最长公共子序列.java)                  |

### UnionFindSet

|      题号      |                  题名                  |  难度  |       简介       |                                        题解                                        |
|:------------:|:------------------------------------:|:----:|:--------------:|:--------------------------------------------------------------------------------:|
| leetcode 990 | satisfiability-of-equality-equations | Easy | 模板题,可以测试自己写的模板 | [java](java/src/leetCode/repository/No990SatisfiabilityOfEqualityEquations.java) |

### BinaryIndexedTree(Fenwick tree)

|      题号      |      题名       |  难度  |         简介          |                                      题解                                       |
|:------------:|:-------------:|:----:|:-------------------:|:-----------------------------------------------------------------------------:|
| leetcode 315 | 计算右侧小于当前元素的个数 | Hard | 树状数组 + 离散化,注意下数据有重复 | [java](java/src/leetCode/repository/No315CountOfSmallerNumbersAfterSelf.java) |

### Graph theory

#### Topological Sorting

|      题号      |  题名   |   难度   |        简介        |                          题解                          |
|:------------:|:-----:|:------:|:----------------:|:----------------------------------------------------:|
| leetcode 210 | 课程表II | Medium | 拓扑排序 + 深度/广度优先遍历 | [java](java/src/leetCode/repository/No210课程表II.java) |

#### Dijkstra

|      题号       |   题名    |   难度   |       简介       |                                     题解                                     |
|:-------------:|:-------:|:------:|:--------------:|:--------------------------------------------------------------------------:|
|  leetcode 45  | 跳跃游戏II  | Medium | 可将图画出来,发现就是最短路 |            [java](java/src/leetCode/repository/No45跳跃游戏II.java)            |
| leetcode 1514 | 概率最大的路径 | Medium |    Dijkstra    | [java](java/src/leetCode/repository/No1514PathWithMaximumProbability.java) |

#### Network flow

|    题号    |      题名      |  难度  |             简介             |                      题解                      |
|:--------:|:------------:|:----:|:--------------------------:|:--------------------------------------------:|
| hdu 3549 | Flow Problem | Easy | 完全模板题,可以测试自己写的网络流模板,但是输入有坑 | [java](/java/src/hdu/No3549FlowProblem.java) |
