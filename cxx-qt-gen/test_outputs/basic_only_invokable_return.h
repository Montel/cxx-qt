#pragma once

#include "rust/cxx_qt.h"

class MyObjectRs;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE int doubleNumber(int number) const;
  Q_INVOKABLE QString helloMessage(const QString& msg) const;
  Q_INVOKABLE QString staticMessage() const;

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();