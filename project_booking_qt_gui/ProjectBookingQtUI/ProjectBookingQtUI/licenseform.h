#ifndef LICENSEFORM_H
#define LICENSEFORM_H
#include <QPushButton>
#include <QTextBrowser>
#include "reports.h"

class LicenseForm: public QWidget
{
    Q_OBJECT

public:
    explicit LicenseForm(Reports *, StatusDisplay *,QWidget *);

public slots:
    void trigger();

private:
    QWidget *parentWidget;
    Reports *reports;
    StatusDisplay *statusDisplay;
    QPushButton *cancel;
    void close();
};

#endif // LICENSEFORM_H
