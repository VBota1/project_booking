#include "projectbooking.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    ProjectBooking w;
    w.show();

    return a.exec();
}
