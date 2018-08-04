#ifndef HELPFORM_H
#define HELPFORM_H
#include <QPushButton>
#include <QTextBrowser>
#include "reports.h"

class HelpForm : public QWidget
{
    Q_OBJECT

public:
    explicit HelpForm(Reports *, StatusDisplay *,QWidget *);

public slots:
    void trigger();

private:
    QWidget *parentWidget;
    Reports *reports;
    StatusDisplay *statusDisplay;
    QPushButton *cancel;
    void close();
};

#endif // HELPFORM_H
