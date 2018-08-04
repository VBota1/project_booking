#include "licenseform.h"

LicenseForm::LicenseForm(Reports *referencedReports, StatusDisplay *status,QWidget *wparent) : QWidget(wparent)
{
    parentWidget=wparent;
    reports = referencedReports;
    statusDisplay = status;
}

void LicenseForm::trigger() {
    BackendHandler backend;
    Result<QString> response = backend.license();
    if(response.hasError)
    {
        statusDisplay->update(response.err());
        return;
    }

    parentWidget->setMaximumWidth(16777215);
    parentWidget->setSizePolicy(QSizePolicy::Expanding,QSizePolicy::Expanding);

    QGridLayout *parentLayout = new QGridLayout(parentWidget);

    QTextBrowser *helpMessage = new QTextBrowser();
    helpMessage->setText(response.ok());
    helpMessage->setMinimumWidth(450);
    parentLayout->addWidget(helpMessage);

    cancel = new QPushButton(tr("Close"));
    connect( cancel, &QPushButton::clicked, this, &LicenseForm::close );
    parentLayout->addWidget(cancel);

    parentWidget->setLayout(parentLayout);
}

void LicenseForm::close() {
    parentWidget->layout()->deleteLater();
    parentWidget->setMaximumWidth(150);
    parentWidget->setSizePolicy(QSizePolicy::Minimum,QSizePolicy::Minimum);
}
