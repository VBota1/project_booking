#ifndef STATUSDISPLAY_H
#define STATUSDISPLAY_H
#include "ui_projectbooking.h"

class StatusDisplay
{
private:
    QListWidget *statusWidget;
public:
    StatusDisplay(QListWidget*);
    void update(QString message);
};

#endif // STATUSDISPLAY_H
