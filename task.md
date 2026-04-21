课后练习
编程题
1. * 扩展内核，能够显示操作系统切换任务的过程。

1. ** 扩展内核，能够统计每个应用执行后的完成时间：用户态完成时间和内核态完成时间。

1. ** 编写浮点应用程序A，并扩展内核，支持面向浮点应用的正常切换与抢占。

1. ** 编写应用程序或扩展内核，能够统计任务切换的大致开销。

1. *** 扩展内核，支持在内核态响应中断。

1. *** 扩展内核，支持在内核运行的任务（简称内核任务），并支持内核任务的抢占式切换。

注：上述扩展内核的编程基于 rcore/ucore tutorial v3: Branch ch3

问答题
1. * 协作式调度与抢占式调度的区别是什么？

1. * 中断、异常和系统调用有何异同之处？

1. * RISC-V支持哪些中断/异常？

1. * 如何判断进入操作系统内核的起因是由于中断还是异常？

1. ** 在 RISC-V 中断机制中，PLIC 和 CLINT 各起到了什么作用？

1. ** 基于RISC-V 的操作系统支持中断嵌套？请给出进一步的解释说明。

1. ** 本章提出的任务的概念与前面提到的进程的概念之间有何区别与联系？

1. * 简单描述一下任务的地址空间中有哪些类型的数据和代码。

1. * 任务控制块保存哪些内容？

1. * 任务上下文切换需要保存与恢复哪些内容？

1. * 特权级上下文和任务上下文有何异同？

1. * 上下文切换为什么需要用汇编语言实现？

1. * 有哪些可能的时机导致任务切换？

1. ** 在设计任务控制块时，为何采用分离的内核栈和用户栈，而不用一个栈？

1. *** 我们已经在 rCore 里实现了不少操作系统的基本功能：特权级、上下文切换、系统调用……为了让大家对相关代码更熟悉，我们来以另一个操作系统为例，比较一下功能的实现。看看换一段代码，你还认不认识操作系统。

阅读 Linux 源代码，特别是 riscv 架构相关的代码，回答以下问题：

Linux 正常运行的时候， stvec 指向哪个函数？是哪段代码设置的 stvec 的值？

Linux 里进行上下文切换的函数叫什么？（对应 rCore 的 __switch ）

Linux 里，和 rCore 中的 TrapContext 和 TaskContext 这两个类型大致对应的结构体叫什么？

Linux 在内核态运行的时候， tp 寄存器的值有什么含义？ sscratch 的值是什么？

Linux 在用户态运行的时候， sscratch 的值有什么含义？

Linux 在切换到内核态的时候，保存了和用户态程序相关的什么状态？

Linux 在内核态的时候，被打断的用户态程序的寄存器值存在哪里？在 C 代码里如何访问？

Linux 是如何根据系统调用编号找到对应的函数的？（对应 rCore 的 syscall::syscall() 函数的功能）

Linux 用户程序调用 ecall 的参数是怎么传给系统调用的实现的？系统调用的返回值是怎样返回给用户态的？

阅读代码的时候，可以重点关注一下如下几个文件，尤其是第一个 entry.S ，当然也可能会需要读到其它代码：

arch/riscv/kernel/entry.S （与 rCore 的 switch.S 对比）

arch/riscv/include/asm/current.h

arch/riscv/include/asm/processor.h

arch/riscv/include/asm/switch_to.h

arch/riscv/kernel/process.c

arch/riscv/kernel/syscall_table.c

arch/riscv/kernel/traps.c

include/linux/sched.h

此外，推荐使用 https://elixir.bootlin.com 阅读 Linux 源码，方便查找各个函数、类型、变量的定义及引用情况。

一些提示：

Linux 支持各种架构，查找架构相关的代码的时候，请认准文件名中的 arch/riscv 。

为了同时兼容 RV32 和 RV64，Linux 在汇编代码中用了几个宏定义。例如， REG_L 在 RV32 上是 lw ，而在 RV64 上是 ld 。同理， REG_S 在 RV32 上是 sw ，而在 RV64 上是 sd 。

如果看到 #ifdef CONFIG_ 相关的预处理指令，是 Linux 根据编译时的配置启用不同的代码。一般阅读代码时，要么比较容易判断出这些宏有没有被定义，要么其实无关紧要。比如，Linux 内核确实应该和 rCore 一样，是在 S-mode 运行的，所以 CONFIG_RISCV_M_MODE 应该是没有启用的。

汇编代码中可能会看到有些 TASK_ 和 PT_ 开头的常量，找不到定义。这些常量并没有直接写在源码里，而是自动生成的。

在汇编语言中需要用到的很多 struct 里偏移量的常量定义可以在 arch/riscv/kernel/asm-offsets.c 文件里找到。其中， OFFSET(NAME, struct_name, field) 指的是 NAME 的值定义为 field 这一项在 struct_name 结构体里，距离结构体开头的偏移量。最终这些代码会生成 asm/asm-offsets.h 供汇编代码使用。

#include <asm/unistd.h> 在 arch/riscv/include/uapi/asm/unistd.h ， #include <asm-generic/unistd.h> 在 include/uapi/asm-generic/unistd.h 。

实验练习
实验练习包括实践作业和问答作业两部分。

实践作业
获取任务信息
ch3 中，我们的系统已经能够支持多个任务分时轮流运行，我们希望引入一个新的系统调用 sys_task_info 以获取任务的信息，定义如下：

fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize
syscall ID: 410

根据任务 ID 查询任务信息，任务信息包括任务 ID、任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、任务总运行时长。

struct TaskInfo {
    id: usize,
    status: TaskStatus,
    call: [SyscallInfo; MAX_SYSCALL_NUM],
    time: usize
}
系统调用信息采用数组形式对每个系统调用的次数进行统计，相关结构定义如下：

struct SyscallInfo {
    id: usize,
    times: usize
}
参数：
id: 待查询任务id

ts: 待查询任务信息

返回值：执行成功返回0，错误返回-1

说明：
相关结构已在框架中给出，只需添加逻辑实现功能需求即可。

提示：
大胆修改已有框架！除了配置文件，你几乎可以随意修改已有框架的内容。

程序运行时间可以通过调用 get_time() 获取。

系统调用次数可以考虑在进入内核态系统调用异常处理函数之后，进入具体系统调用函数之前维护。

阅读 TaskManager 的实现，思考如何维护内核控制块信息（可以在控制块可变部分加入其他需要的信息）

打印调用堆栈（选做）
我们在调试程序时，除了正在执行的函数外，往往还需要知道当前的调用堆栈。这样的功能通常由调试器、运行环境、 IDE 或操作系统等提供，但现在我们只能靠自己了。最基本的实现只需打印出调用链上的函数地址，更丰富的功能包括打印出函数名、函数定义、传递的参数等等。

本实验我们不提供新的测例，仅提供参考实现，各位同学可以通过对照 GDB 、参考实现或自行构造调用链等方式检验自己的实现是否正确。

