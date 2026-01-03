import flet as ft

BASE_COLOR = ft.Colors.BLUE_GREY_400
HIGHLIGHT_COLOR = ft.Colors.BLUE_200

PAINT_COLORS = {
    "Красный": ft.Colors.RED_400,
    "Синий": ft.Colors.BLUE_400,
    "Зелёный": ft.Colors.GREEN_400,
    "Жёлтый": ft.Colors.YELLOW_400,
}

RIGHT_CONFIG = {
    "Золото":      {"count": 16, "arrows": [4, 8, 11, 14, 16]},
    "Нефть":       {"count": 16, "arrows": [4, 8, 12, 16]},
    "Nasdaq":      {"count": 15, "arrows": [3, 6, 9, 12, 15]},
    "Dow Jones":   {"count": 14, "arrows": [4, 8, 11, 14]},
    "Облигации":   {"count": 15, "arrows": [5, 10, 15]},
    "Акции стран": {"count": 12, "arrows": [2, 5, 8, 12]},
}

def main(page: ft.Page):
    page.title = "Хулиполия"
    page.bgcolor = ft.Colors.BLACK
    page.scroll = ft.ScrollMode.AUTO

    selected_color = {"value": ft.Colors.RED_400}

    wife_money = {"value": 0}
    husband_money = {"value": 0}

    def apply_money(change_field, money_state, money_text):
        try:
            delta = int(change_field.value)
            money_state["value"] += delta
            money_text.value = str(money_state["value"])
            change_field.value = ""
            page.update()
        except:
            pass

    palette = ft.Row(spacing=10)

    def select_color(color):
        selected_color["value"] = color
        for b in palette.controls:
            b.border = ft.border.all(
                3,
                ft.Colors.WHITE if b.bgcolor == color else ft.Colors.TRANSPARENT,
            )
        palette.update()

    for color in PAINT_COLORS.values():
        palette.controls.append(
            ft.Container(
                width=40,
                height=40,
                bgcolor=color,
                border_radius=6,
                border=ft.border.all(3, ft.Colors.TRANSPARENT),
                on_click=lambda e, c=color: select_color(c),
            )
        )

    def paint(e):
        cell = e.control
        if cell.bgcolor == selected_color["value"]:
            cell.bgcolor = (
                HIGHLIGHT_COLOR if getattr(cell, "highlight", False) else BASE_COLOR
            )
        else:
            cell.bgcolor = selected_color["value"]
        cell.update()

    def make_left_scale():
        cells = []
        for i in range(18):
            highlight = i in [0, 1, 2, 3, 16, 17]
            c = ft.Container(
                content=ft.Text(str(i), color="white"),
                width=32,
                height=32,
                alignment=ft.alignment.center,
                bgcolor=HIGHLIGHT_COLOR if highlight else BASE_COLOR,
                border_radius=4,
                on_click=paint,
            )
            c.highlight = highlight
            cells.append(c)
        return ft.Row(cells, spacing=4)

    def make_arrow_rows(count, arrows):
        rows = []
        for _ in range(2):
            cells = []
            for i in range(1, count + 1):
                is_arrow = i in arrows
                margin = ft.margin.only(right=10) if is_arrow else None
                cells.append(
                    ft.Container(
                        content=ft.Text("↗" if is_arrow else "", color="white"),
                        width=32,
                        height=32,
                        alignment=ft.alignment.center,
                        bgcolor=BASE_COLOR,
                        border_radius=4,
                        on_click=paint,
                        margin=margin,
                    )
                )
            rows.append(ft.Row(cells, spacing=4))
        return ft.Column(rows, spacing=4)

    rows = []

    for title, cfg in RIGHT_CONFIG.items():
        left = make_left_scale()
        arrows = make_arrow_rows(cfg["count"], cfg["arrows"])

        rows.append(
            ft.Row(
                [
                    left,
                    ft.Text(title, width=160, color="white", weight="bold"),
                    arrows,
                ],
                alignment="start",
            )
        )

    wife_money_text = ft.Text("0", color="white", size=16)
    wife_change = ft.TextField(label="+ / - деньги", width=150)

    husband_money_text = ft.Text("0", color="white", size=16)
    husband_change = ft.TextField(label="+ / - деньги", width=150)

    page.add(
        ft.Column(
            [
                ft.Text("Счётчики игроков", color="white", size=18, weight="bold"),

                ft.Row(
                    [
                        ft.Column(
                            [
                                ft.Text("Жена", color="white", weight="bold"),
                                ft.TextField(label="Кредиты", width=150),
                                ft.Row([ft.Text("Деньги:", color="white"), wife_money_text]),
                                ft.Row(
                                    [
                                        wife_change,
                                        ft.ElevatedButton(
                                            "Применить",
                                            on_click=lambda e: apply_money(
                                                wife_change, wife_money, wife_money_text
                                            ),
                                        ),
                                    ]
                                ),
                            ]
                        ),

                        ft.Column(
                            [
                                ft.Text("Муж", color="white", weight="bold"),
                                ft.TextField(label="Кредиты", width=150),
                                ft.Row([ft.Text("Деньги:", color="white"), husband_money_text]),
                                ft.Row(
                                    [
                                        husband_change,
                                        ft.ElevatedButton(
                                            "Применить",
                                            on_click=lambda e: apply_money(
                                                husband_change, husband_money, husband_money_text
                                            ),
                                        ),
                                    ]
                                ),
                            ]
                        ),
                    ],
                    spacing=40,
                ),

                ft.Divider(color="white"),

                ft.Text("Выбери цвет:", color="white"),
                palette,
                ft.Divider(color="white"),
                *rows,
            ],
            spacing=14,
        )
    )

    select_color(selected_color["value"])

ft.app(target=main, view=ft.WEB_BROWSER)
