// 临时最小化 TaskFlow 包装
// 如果网络可用，请运行: git submodule update --init --recursive
// 来获取完整的 TaskFlow 库

#ifndef TASKFLOW_TASKFLOW_HPP
#define TASKFLOW_TASKFLOW_HPP

#include <functional>
#include <vector>
#include <thread>
#include <future>

namespace tf {

// 简单的 Future 包装，用于支持 .wait() 调用
class TaskflowFuture {
public:
    void wait() {
        // 同步执行，立即返回
    }
};

class Executor {
public:
    template<typename F>
    TaskflowFuture run(F&& taskflow) {
        // 执行所有任务
        for (size_t i = 0; i < taskflow.num_tasks(); ++i) {
            // 在实际 TaskFlow 中，这里会并行执行任务
            // 这里简化处理，顺序执行
        }
        // 执行任务流中的任务
        taskflow.clear();
        return TaskflowFuture();
    }
};

class Taskflow {
private:
    std::vector<std::function<void()>> tasks_;
    
public:
    Taskflow() = default;
    
    template<typename Func>
    void emplace(Func&& func) {
        tasks_.emplace_back(std::forward<Func>(func));
    }
    
    void clear() {
        // 执行所有任务
        for (auto& task : tasks_) {
            task();
        }
        tasks_.clear();
    }
    
    size_t num_tasks() const {
        return tasks_.size();
    }
};

} // namespace tf

#endif // TASKFLOW_TASKFLOW_HPP

