from fastapi import FastAPI
from .routes.prediction_route import router as prediction_router
from fastapi.middleware.cors import CORSMiddleware


app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["GET", "POST", "PUT", "PATCH", "DELETE"],
    allow_headers=["*"],
)

app.include_router(prediction_router, tags=["Predição e Detecção - Dashboard UPA API"])


