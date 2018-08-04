#ifndef RESULT_H
#define RESULT_H
#include "reportformats.h"

template <class T>
class Result
{
private:
    T ok_value;
    QString err_value;
public:
    bool hasError;

    void err(QString value){
        err_value = value;
        hasError = true;
    };

    QString err(){
        return err_value;
    };

    void ok(T value){
        ok_value = value;
        hasError = false;
    };

    T ok(){
        return ok_value;
    };

    Result() {};
};

#endif // RESULT_H
