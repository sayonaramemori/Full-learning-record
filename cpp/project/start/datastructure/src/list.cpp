#include <iostream>
#include <functional>
#include <optional>
#define LOG(x) std::cout<< x <<std::endl

//T should suppose copy constructor
template<typename T>
class List 
{
public:
    List():head(nullptr),tail(nullptr){}
    ~List(){
        for(auto cur=this->head;cur!=nullptr;){
            auto next = cur->next;
            delete cur;
            cur = next;
        }
    }

    typedef struct ListNode
    {
        ListNode(const T& v):pre(nullptr),next(nullptr),val(v){}
        ListNode* pre;
        ListNode* next;
        T val;
    };

    void reverse() 
    {
        std::swap(head,tail);
    }

    T& append(const T& temp){
        ListNode* tail = new ListNode(temp);
        if(this->head==nullptr){
            this->head = this->tail = tail;
        }else{
            tail->pre = this->tail;
            this->tail->next = tail;
            this->tail = tail;
        }
        return tail->val;
    }

    std::optional<std::vector<T>> remove_if(std::function<bool(const T&)> fn){
        std::vector<T> res = {};
        for(ListNode* cur = head;cur!=nullptr;cur=cur->next)
        {
            if(fn(cur->val)){
                res.push_back(cur->val);
                if(cur==head){
                    head->next->pre = nullptr;
                    head = head->next;
                    delete cur;
                    cur = head;
                }else if(cur==tail){
                    tail = tail->pre;
                    tail->next = nullptr;
                    delete cur;
                    cur = tail;
                }else{
                    auto pre = cur->pre;
                    pre->next = cur->next;
                    cur->next->pre = pre;
                    delete cur;
                    cur = pre;
                }
            }
        }
        if(res.empty())return std::nullopt;
        else return {res};
    }

    std::optional<std::vector<T>> remove_n_if(std::function<bool(const T&)> fn, size_t n){
        std::vector<T> res = {};
        for(ListNode* cur = head;cur!=nullptr;cur=cur->next)
        {
            if(fn(cur->val) && n>0){
                res.push_back(cur->val);
                if(cur==head){
                    head->next->pre = nullptr;
                    head = head->next;
                    delete cur;
                    cur = head;
                }else if(cur==tail){
                    tail = tail->pre;
                    tail->next = nullptr;
                    delete cur;
                    cur = tail;
                }else{
                    auto pre = cur->pre;
                    pre->next = cur->next;
                    cur->next->pre = pre;
                    delete cur;
                    cur = pre;
                }
                --n;
            }
        }       
        if(res.empty())return std::nullopt;
        else return {res};
    }

    std::optional<T*> insert_after(const T& target,const T& val) {
        for(auto cur=head;cur!=nullptr;cur = cur->next)
        {
            if(cur->val == target){
                ListNode* node = new ListNode(val);
                if(cur==tail){
                    tail->next = node;
                    tail = node;
                    tail->pre = cur;
                }else{
                    node->next = cur->next;
                    node->next->pre = node;
                    node->pre = cur;
                    cur->next = node;
                }
                return {&(node->val)};
            }
        }
        return std::nullopt;
    }

    std::optional<T*> insert_before(const T& target,const T& val) {
        for(auto cur=head;cur!=nullptr;cur = cur->next)
        {
            if(cur->val == target){
                ListNode* node = new ListNode(val);
                if(cur==head){
                    head = node;
                    head->next = cur;
                    cur->pre = head;
                }else{
                    ListNode* pre = cur->pre;
                    node->pre = pre;
                    node->next = cur;
                    pre->next = node;
                    cur->pre = node;
                }
                return {&(node->val)};
            }
        }
        return std::nullopt;
    }

private:
    ListNode* head;
    ListNode* tail;
    template<typename U> friend std::ostream& operator<<(std::ostream& os, List<U>& list); 
};

template<typename T>
std::ostream& operator<<(std::ostream& os, List<T>& list)
{
    for(auto cur = list.head;cur!=nullptr;cur = cur->next){
        os<< cur->val << " -> ";
    }
    return os;
}

// class test 
// { public: test(int n):a(n),b(n){} test(const test& t){ a = t.a; b = t.b; } int value() const{ return a*b; } bool operator==(const test& right) const{ return this->a == right.a || this->b == right.b; }
// private: int a,b; friend std::ostream& operator<<(std::ostream& os, const test& tee); };

// std::ostream& operator<<(std::ostream& os, const test& tee)
// {
//     os<< tee.a * tee.b;
//     return os;
// }

int main() 
{
    List<int> a;
    for(int i=1;i<19;++i)
    {
        a.append(i);
    }
    int target = 9;
    int val = 999;
    a.insert_before(target,val);
    a.insert_after(target,val);
    auto res = a.remove_if([](const int& v){return v%2==0;});

    a.reverse();
    std::cout<<a<<std::endl;
    for(auto &i:res.value_or(std::vector<int>{})){
        std::cout<<i<<std::endl;
    }
    if(int a=10){
        std::cout<<a<<std::endl;
    }
        std::cout<<a<<std::endl;
}
