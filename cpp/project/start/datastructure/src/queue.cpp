// #define INCLUDE
#ifdef INCLUDE
template<typename Type,unsigned N>
class Queue{
    Type* items = {nullptr};
    unsigned rear = {0};
    unsigned head = {0};
public:
    Queue();
    ~Queue();
    void push(const Type&);
    void push(Type*);
    Type shift();
    Type* shift_ptr();
};


template<typename Type,unsigned N>
Queue<Type,N>::Queue(){
    this->items = new Type[N];
}

template<typename Type,unsigned N>
Queue<Type,N>::~Queue(){

}

template<typename Type,unsigned N>
void Queue<Type,N>::push(const Type& val){
    this->items[rear++] = new Type(val);
    rear = rear%N;
}

template<typename Type,unsigned N>
void Queue<Type,N>::shift(){
    this->items[rear++] = new Type(val);
    rear = rear%N;
}

#endif
