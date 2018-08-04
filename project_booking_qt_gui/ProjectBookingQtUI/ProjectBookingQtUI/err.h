#ifndef ERR_H
#define ERR_H
#include "result.h"

template <class T>
class Err: public Result<T>
{
public:
    Err(QString value){
        this->err(value);
    };
};

#endif // ERR_H
