#include <iostream>
#include <functional>
#include <optional>
#define LOG(x) std::cout<< x <<std::endl

//T should suppose copy constructor
template<typename T>
class List 
{
    class ListNode
    {
    public:
        ListNode(const T& v):val(v){}
        ListNode(const ListNode& v) = delete;
        ListNode() = delete;
        ListNode& operator=(const ListNode& v) = delete;
        ListNode& operator=(ListNode&& v) = delete;
        ListNode* pre = nullptr;
        ListNode* next = nullptr;
        T val;
    };

private:
    void destory()
    {
        for(auto cur=this->head;cur!=nullptr;){
            auto next = cur->next;
            delete cur;
            cur = next;
        }
        this->head = nullptr;
        this->tail = nullptr;
    }
public:
    List(){}
    ~List(){
        this->destory();
    }
    // Important: remember to initialize  
    List(const List& rhv)
    {
        LOG("COPY constructor");
        for(ListNode* cur = rhv.head;cur!=nullptr;cur=cur->next)
        {
            this->append(cur->val);
        }
    }
    List(List&& rhv) 
    {
        LOG("MOVE constructor");
        this->head = rhv.head;
        this->tail = rhv.tail;
        rhv.tail = rhv.head = nullptr;
    }
    List& operator=(const List& rhv)
    {
        if(&rhv == this)return *this;
        this->destory();
        LOG("COPY Assgin");
        for(ListNode* cur = rhv.head;cur!=nullptr;)
        {
            this->append(cur->val);
            cur=cur->next;
        }
    }
    List& operator=(List&& rhv)
    {
        if(&rhv == this)return *this;
        this->destory();
        LOG("MOVE Assgin");
        this->head = rhv.head;
        this->tail = rhv.tail;
        rhv.tail = rhv.head = nullptr;
    }
    
    T& append(const T& temp){
        ListNode* new_tail = new ListNode(temp);
        if(this->head==nullptr){
            this->head = this->tail = new_tail;
        }else{
            new_tail->pre = this->tail;
            this->tail->next = new_tail;
            this->tail = new_tail;
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
    ListNode* head = nullptr;
    ListNode* tail = nullptr;
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

// int main() 
// {
//     List<int> a;
//     for(int i=1;i<19;++i)
//     {
//         a.append(i);
//     }
//     int target = 9;
//     int val = 999;
//     a.insert_before(target,val);
//     a.insert_after(target,val);
//     auto res = a.remove_if([](const int& v){return v%2==0;});

//     std::cout<<a<<std::endl;
//     List<int> b = a;
//     std::cout<<b<<std::endl;

//     //for(auto &i:res.value_or(std::vector<int>{})){ std::cout<<i<<std::endl; }
// }
