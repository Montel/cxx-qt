// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QPolygon>
#include <QtTest/QTest>

#include "cxx-qt-gen/qpolygon.cxx.h"

class QPolygonTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qpolygon();
    QVERIFY(m.isEmpty());
  }
  void clone()
  {
    const auto m = QPolygon(QList() << QPoint(1, 2) << QPoint(3, 4));
    const auto c = clone_qpolygon(m);
    QCOMPARE(c.point(0), { QPoint(1, 2) });
    QCOMPARE(c.point(1), QPoint(3, 4));
  }
};
