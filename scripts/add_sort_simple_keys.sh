#!/usr/bin/env bash
# Phase 16 — simple sort preset names for the activity list.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF
sort-name-asc-simple = $1
sort-name-desc-simple = $2
sort-size-asc-simple = $3
sort-size-desc-simple = $4
EOF
}

append ar "الاسم أ ← ي" "الاسم ي ← أ" "الأصغر حجمًا أولًا" "الأكبر حجمًا أولًا"
append de "Name A → Z" "Name Z → A" "Kleinste zuerst" "Größte zuerst"
append es "Nombre A → Z" "Nombre Z → A" "Más pequeños primero" "Más grandes primero"
append fr "Nom A → Z" "Nom Z → A" "Plus petits d'abord" "Plus grands d'abord"
append hi "नाम A → Z" "नाम Z → A" "सबसे छोटा पहले" "सबसे बड़ा पहले"
append id "Nama A → Z" "Nama Z → A" "Terkecil dulu" "Terbesar dulu"
append it "Nome A → Z" "Nome Z → A" "Più piccoli prima" "Più grandi prima"
append ja "名前 A → Z" "名前 Z → A" "小さい順" "大きい順"
append ko "이름 A → Z" "이름 Z → A" "작은 순" "큰 순"
append nl "Naam A → Z" "Naam Z → A" "Kleinste eerst" "Grootste eerst"
append pl "Nazwa A → Z" "Nazwa Z → A" "Najmniejsze najpierw" "Największe najpierw"
append pt-BR "Nome A → Z" "Nome Z → A" "Menores primeiro" "Maiores primeiro"
append ru "Имя А → Я" "Имя Я → А" "Меньшие первыми" "Большие первыми"
append tr "Ad A → Z" "Ad Z → A" "Küçükten büyüğe" "Büyükten küçüğe"
append uk "Ім'я А → Я" "Ім'я Я → А" "Менші першими" "Більші першими"
append vi "Tên A → Z" "Tên Z → A" "Nhỏ nhất trước" "Lớn nhất trước"
append zh-CN "名称 A → Z" "名称 Z → A" "从小到大" "从大到小"

echo "Activity sort preset keys added to 17 locales."
