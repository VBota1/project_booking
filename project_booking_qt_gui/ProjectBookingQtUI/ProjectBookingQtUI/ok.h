#ifndef OK_H
#define OK_H
#include "result.h"

template <class T>
class Ok: public Result<T>
{
public:
    Ok(T value){
        this->ok(value);
    };
};

#endif // OK_H
